// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate objc;

mod shader_compiler;
mod shader_file_path_arg;

use anyhow::{Context, Result};

use cocoa::{appkit::NSView, base::id as cocoa_id};

use metal::*;
use objc::{rc::autoreleasepool, runtime::YES};
use std::mem;
use winit::platform::macos::WindowExtMacOS;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

// Used for passing per-frame input to pixel shader and for setting up the vertex shader
#[repr(C)]
#[derive(Copy, Clone)]
struct Float2 {
    pub x: f32,
    pub y: f32,
}
#[repr(C)]
#[derive(Copy, Clone)]
struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[repr(C)]
struct Input {
    window_size: Float2,
    window_position: Float2,
    cursor_position: Float2,
    is_cursor_inside_window: f32,
    elapsed_time_secs: f32,
    elapsed_time_since_last_frame_secs: f32,
    frame_count: f32,
    year_month_day_tz: Float4,
}

enum CLICommand {
    Pause,
    Restart,
    Run(String),
}

fn main() -> Result<()> {
    let events_loop = winit::event_loop::EventLoop::new();
    let (window_size, window_position) = window_sizing((0.4, 0.4), &events_loop);
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(window_size)
        .with_outer_position(window_position)
        .with_title("ShaderRoy")
        .with_always_on_top(true)
        .build(&events_loop)
        .unwrap();

    let device = Device::system_default().expect("no device found");

    let layer = MetalLayer::new();
    layer.set_device(&device);
    layer.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    layer.set_presents_with_transaction(false);

    unsafe {
        let view = window.ns_view() as cocoa_id;
        view.setWantsLayer(YES);
        view.setLayer(mem::transmute(layer.as_ref()));
    }

    let draw_size = window.inner_size();
    layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

    let vector_rect = vec![Float4 {
        x: -1.0,
        y: -1.0,
        z: 2.0,
        w: 2.0,
    }];

    let vector_buffer = device.new_buffer_with_data(
        vector_rect.as_ptr() as *const _,
        mem::size_of::<Float4>() as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    );

    let command_queue = device.new_command_queue();
    let mut run_error: Option<()> = None;

    let mut shader_file_path = shader_file_path_arg::get_path()?;
    let mut shader_files_watcher = ShaderFilesWatcher::new(std::time::Duration::from_secs(1))?;
    shader_files_watcher.watch(&shader_file_path)?;
    let cli_commands_watcher = CLICommandsWatcher::new();
    let mut pipeline_state: Option<RenderPipelineState> = None;
    let mut frame_rate_reporter = FrameRateReporter::new();
    let mut input_computer = InputComputer::new();

    events_loop.run(move |event, _, control_flow| {
        autoreleasepool(|| {
            let res = (|| -> Result<(), Box<dyn std::error::Error>> {
                *control_flow = ControlFlow::WaitUntil(
                    std::time::Instant::now() + std::time::Duration::from_secs_f64(1.0 / 60.0),
                );

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(size) => {
                            layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                        }
                        WindowEvent::Focused(is_focused) => {
                            input_computer.set_is_focused(is_focused);
                        }
                        WindowEvent::CursorEntered { .. } => {
                            input_computer.set_cursor_presence(true);
                        }
                        WindowEvent::CursorLeft { .. } => {
                            input_computer.set_cursor_presence(false);
                        }
                        WindowEvent::CursorMoved { position, .. } => {
                            input_computer.set_cursor_position(position);
                        }
                        WindowEvent::MouseInput { state, button, .. } => {
                            input_computer.set_cursor_click_state(button, state)
                        }
                        _ => (),
                    },
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        match cli_commands_watcher.try_recv() {
                            Some(CLICommand::Pause) => {
                                input_computer.toggle_paused();
                            }
                            Some(CLICommand::Restart) => {
                                input_computer.reset();
                            }
                            Some(CLICommand::Run(new_path)) => {
                                shader_file_path =
                                    shader_file_path_arg::get_path_for_argument(&new_path)?;
                                shader_files_watcher.watch(&shader_file_path)?;
                                pipeline_state = None;
                                input_computer.reset();
                            }
                            None => {}
                        }
                        if input_computer.is_paused() {
                            return Ok(());
                        }

                        if pipeline_state.is_none() && run_error.is_none()
                            || shader_files_watcher.has_changes()
                        {
                            let library = shader_compiler::compile_shader(
                                &shader_file_path,
                                &device,
                                |fragment_shader_in_msl| {
                                    println!("{}", fragment_shader_in_msl);
                                    CLICommandsWatcher::print_instructions();
                                },
                            )?;
                            pipeline_state = Some(prepare_pipeline_state(
                                &device,
                                &library,
                                "vertex_shader",
                                "fragment_shader",
                            )?);
                        }
                        if pipeline_state.is_none() {
                            return Ok(());
                        }

                        let render_pass_descriptor = RenderPassDescriptor::new();

                        let drawable = layer.next_drawable().ok_or("No drawable")?;
                        prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

                        let command_buffer = command_queue.new_command_buffer();
                        let encoder =
                            command_buffer.new_render_command_encoder(&render_pass_descriptor);

                        encoder.set_render_pipeline_state(pipeline_state.as_ref().unwrap());
                        encoder.set_vertex_buffer(0, Some(&vector_buffer), 0);
                        encoder.set_fragment_bytes(
                            0,
                            std::mem::size_of::<Input>() as u64,
                            &input_computer.current_input(&window) as *const Input as *const _,
                        );
                        encoder.draw_primitives_instanced(
                            metal::MTLPrimitiveType::TriangleStrip,
                            0,
                            4,
                            1,
                        );

                        encoder.end_encoding();
                        command_buffer.present_drawable(&drawable);
                        command_buffer.commit();

                        frame_rate_reporter.calculate_frame_rate_and_maybe_report(&window);
                    }
                    _ => {}
                };
                Ok(())
            })();
            if let Err(err) = res {
                println!("{:?}", err);
                run_error = Some(());
            }
        });
    });
}

struct CLICommandsWatcher {
    #[allow(dead_code)] // Required to avoid dropping child
    watcher_thread: std::thread::JoinHandle<()>,
    receiver: std::sync::mpsc::Receiver<CLICommand>,
}

impl CLICommandsWatcher {
    fn new() -> Self {
        let (tx, receiver) = std::sync::mpsc::channel();
        let watcher_thread = std::thread::spawn(move || loop {
            let mut cli_input = String::new();
            let _ = std::io::stdin().read_line(&mut cli_input);
            let event = match &cli_input[0..1] {
                "p" => CLICommand::Pause,
                "r" => match &cli_input[1..2] {
                    " " => CLICommand::Run(cli_input[2..cli_input.len() - 1].to_owned()),
                    _ => CLICommand::Restart,
                },
                _ => {
                    continue;
                }
            };
            Self::print_prompt();
            tx.send(event).unwrap();
        });

        CLICommandsWatcher {
            watcher_thread,
            receiver,
        }
    }

    fn try_recv(&self) -> Option<CLICommand> {
        self.receiver.try_recv().ok()
    }

    fn print_instructions() {
        println!("[p] to pause, [r] to restart, [r <path>] to run another shader ");
        Self::print_prompt();
    }

    fn print_prompt() {
        print!("> ");
        use std::io::Write;
        let _ = std::io::stdout().flush();
    }
}

struct InputComputer {
    start_time: std::time::Instant,
    last_frame_time: std::time::Instant,
    frame_count: u64,
    is_window_focused: bool,
    is_cursor_inside_window: bool,
    cursor_position: Float2,
    pointer_device_state: Float2,
    date: Float4,
    pause_time: Option<std::time::Instant>,
}

impl InputComputer {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            last_frame_time: std::time::Instant::now(),
            frame_count: 0,
            is_window_focused: true,
            is_cursor_inside_window: false,
            cursor_position: Float2 { x: 0.0, y: 0.0 },
            pointer_device_state: Float2 { x: 0.0, y: 0.0 },
            date: current_date_as_float4(),
            pause_time: None,
        }
    }

    fn reset(&mut self) {
        self.start_time = std::time::Instant::now();
        self.last_frame_time = std::time::Instant::now();
        self.frame_count = 0;
        self.date = current_date_as_float4();
        self.pause_time = None;
    }

    fn toggle_paused(&mut self) {
        if let Some(pause_time) = self.pause_time {
            let pause_duration = pause_time.elapsed();
            self.start_time += pause_duration;
            self.last_frame_time = std::time::Instant::now();
            self.pause_time = None;
        } else {
            self.pause_time = Some(std::time::Instant::now());
        }
    }

    fn is_paused(&self) -> bool {
        self.pause_time.is_some()
    }

    fn set_is_focused(&mut self, is_focused: bool) {
        self.is_window_focused = is_focused;
    }

    fn set_cursor_presence(&mut self, is_present: bool) {
        self.is_cursor_inside_window = self.is_window_focused && is_present;
    }

    fn set_cursor_position(&mut self, position: winit::dpi::PhysicalPosition<f64>) {
        self.is_cursor_inside_window = true;
        self.cursor_position = Float2 {
            x: position.x as f32,
            y: position.y as f32,
        };
    }

    fn set_cursor_click_state(
        &mut self,
        button: winit::event::MouseButton,
        state: winit::event::ElementState,
    ) {
        if !self.is_cursor_inside_window {
            return;
        }
        let state_value = match state {
            winit::event::ElementState::Pressed => 1.0,
            winit::event::ElementState::Released => 0.0,
        };
        match button {
            winit::event::MouseButton::Left => {
                self.pointer_device_state.x = state_value;
            }
            winit::event::MouseButton::Right => {
                self.pointer_device_state.y = state_value;
            }
            _ => {}
        }
    }

    fn current_input(&mut self, window: &winit::window::Window) -> Input {
        let physical_size = window.inner_size();
        let physical_position = window.inner_position().unwrap();
        self.frame_count += 1;
        let result = Input {
            window_size: Float2 {
                x: physical_size.width as f32,
                y: physical_size.height as f32,
            },
            window_position: Float2 {
                x: physical_position.x as f32,
                y: physical_position.y as f32,
            },
            cursor_position: self.cursor_position,
            is_cursor_inside_window: if self.is_cursor_inside_window {
                1.0
            } else {
                0.0
            },
            elapsed_time_since_last_frame_secs: self.last_frame_time.elapsed().as_secs_f32(),
            elapsed_time_secs: self.start_time.elapsed().as_secs_f32(),
            frame_count: self.frame_count as f32,
            year_month_day_tz: self.date,
        };
        self.last_frame_time = std::time::Instant::now();
        result
    }
}

fn current_date_as_float4() -> Float4 {
    let today = chrono::prelude::Local::now();
    use chrono::Datelike;
    Float4 {
        x: today.year() as f32,
        y: today.month() as f32,
        z: today.day() as f32,
        w: today.offset().local_minus_utc() as f32,
    }
}

fn prepare_pipeline_state(
    device: &DeviceRef,
    library: &LibraryRef,
    vertex_shader: &str,
    fragment_shader: &str,
) -> Result<RenderPipelineState, Box<dyn std::error::Error>> {
    let vert = library.get_function(vertex_shader, None)?;
    let frag = library.get_function(fragment_shader, None)?;

    let pipeline_state_descriptor = RenderPipelineDescriptor::new();
    pipeline_state_descriptor.set_vertex_function(Some(&vert));
    pipeline_state_descriptor.set_fragment_function(Some(&frag));
    let attachment = pipeline_state_descriptor
        .color_attachments()
        .object_at(0)
        .ok_or("No attachment")?;
    attachment.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    Ok(device.new_render_pipeline_state(&pipeline_state_descriptor)?)
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();
    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.0, 0.0, 0.0, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}

fn window_sizing(
    scale: (f32, f32),
    events_loop: &winit::event_loop::EventLoop<()>,
) -> (
    winit::dpi::LogicalSize<f32>,
    winit::dpi::LogicalPosition<f32>,
) {
    let size_scale: vek::Vec2<_> = scale.into();
    let screen = events_loop.primary_monitor().unwrap();
    let screen_size: vek::Vec2<f32> = {
        let size: (f32, f32) = screen
            .size()
            .to_logical::<f32>(screen.scale_factor())
            .into();
        size.into()
    };
    let window_size = (screen_size * size_scale).into_tuple().into();
    let window_position = ((screen_size * (vek::Vec2::from(1.0) - size_scale)).into_tuple()).into();
    (window_size, window_position)
}

struct ShaderFilesWatcher {
    watcher: notify::RecommendedWatcher,
    receiver: std::sync::mpsc::Receiver<notify::DebouncedEvent>,
    watched_path: Option<std::path::PathBuf>,
}

impl ShaderFilesWatcher {
    fn new(delay: std::time::Duration) -> Result<Self> {
        let (tx, receiver) = std::sync::mpsc::channel();
        let watcher = notify::watcher(tx, delay)?;
        let mut files_watcher = ShaderFilesWatcher {
            watcher,
            receiver,
            watched_path: None,
        };
        files_watcher.start_watching(&*shader_compiler::SHADER_PRELUDE_PATH)?;
        files_watcher.start_watching(&*shader_compiler::SHADER_INTERFACE_PATH)?;
        Ok(files_watcher)
    }

    fn watch(&mut self, path: &std::path::Path) -> Result<()> {
        use notify::Watcher;
        if let Some(watched_path) = &self.watched_path {
            let _ = self.watcher.unwatch(watched_path);
        } else {
            self.start_watching(path)?;
            self.watched_path = Some(path.to_owned());
        }
        Ok(())
    }

    fn has_changes(&self) -> bool {
        self.receiver.try_recv().is_ok()
    }

    fn start_watching(&mut self, path: &std::path::Path) -> Result<()> {
        use notify::Watcher;
        self.watcher
            .watch(path, notify::RecursiveMode::Recursive)
            .with_context(|| format!("Failed to watch path {:?}", path))?;
        Ok(())
    }
}

struct FrameRateReporter {
    frame_start_time: std::time::Instant,
    frame_rate_in_frames_per_sec: f64,
    rate_limiter: RateLimiter,
}

impl FrameRateReporter {
    fn new() -> Self {
        Self {
            frame_start_time: std::time::Instant::now(),
            frame_rate_in_frames_per_sec: 30.0, // initial guess
            rate_limiter: RateLimiter::new(std::time::Duration::from_secs(1)),
        }
    }

    fn calculate_frame_rate_and_maybe_report(&mut self, window: &winit::window::Window) {
        let frame_duration = self.frame_start_time.elapsed().as_millis().max(1) as f64;
        let num_frames_averaged = 10.0;
        self.frame_rate_in_frames_per_sec +=
            (1000.0 / frame_duration - self.frame_rate_in_frames_per_sec) / num_frames_averaged;
        let frame_rate_in_frames_per_sec = self.frame_rate_in_frames_per_sec;
        self.rate_limiter.maybe_call(|| {
            window.set_title(&format!(
                "ShaderRoy [FPS: {:.0}]",
                frame_rate_in_frames_per_sec
            ));
            use std::io::Write;
            let _ = std::io::stdout().lock().flush();
        });
        self.frame_start_time = std::time::Instant::now();
    }
}

struct RateLimiter {
    delay: std::time::Duration,
    last_call_time: std::time::Instant,
}

impl RateLimiter {
    fn new(delay: std::time::Duration) -> Self {
        Self {
            delay,
            last_call_time: std::time::Instant::now(),
        }
    }

    fn maybe_call<F: FnOnce()>(&mut self, func: F) {
        if self.last_call_time.elapsed() > self.delay {
            func();
            self.last_call_time = std::time::Instant::now();
        }
    }
}

#[test]
fn test() -> Result<()> {
    let examples_dir = shader_compiler::ROOT_PATH.join("../examples");
    for example in std::fs::read_dir(examples_dir)? {
        let example_name = example?.file_name().into_string().unwrap();
        // Needs leading ../ because tests run as if invoked from this package directory
        let path =
            shader_file_path_arg::get_path_for_argument(&format!("../examples/{}", example_name))?;
        shader_compiler::compile_shader(
            &path,
            &metal::Device::system_default().unwrap(),
            |shader_in_msl| println!("{:?}\n\n{}", &path, shader_in_msl),
        )?;
    }
    Ok(())
}
