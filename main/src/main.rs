// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate objc;

use anyhow::Result;
use metal_hot_reload::*;

use cocoa::{appkit::NSView, base::id as cocoa_id};

use metal::*;
use objc::{rc::autoreleasepool, runtime::YES};
use std::mem;
use winit::platform::macos::WindowExtMacOS;

use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

#[repr(C)]
struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[repr(C)]
struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[repr(C)]
struct ClearRect {
    pub rect: Rect,
    pub color: Color,
}

#[repr(C)]
struct Input {
    pub width: u32,
    pub height: u32,
    pub elapsed_time_secs: f32,
}

fn main() -> Result<()> {
    let events_loop = winit::event_loop::EventLoop::new();
    let (window_size, window_position) = window_sizing((0.4, 0.4), &events_loop);
    let window = winit::window::WindowBuilder::new()
        .with_inner_size(window_size)
        .with_title("ShaderRoy")
        .with_always_on_top(true)
        .build(&events_loop)
        .unwrap();
    window.set_outer_position(window_position);

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

    let vector_rect = vec![ClearRect {
        rect: Rect {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        },
        color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        },
    }];

    let vector_buffer = device.new_buffer_with_data(
        vector_rect.as_ptr() as *const _,
        mem::size_of::<ClearRect>() as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    );

    let command_queue = device.new_command_queue();
    let mut run_error: Option<()> = None;

    // Watch source file
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher = notify::watcher(tx, std::time::Duration::from_secs(1)).unwrap();
    use notify::Watcher;
    watcher
        .watch(
            &*shader_compiler::SHADER_PATH,
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();
    watcher
        .watch(
            &*shader_compiler::SHADER_PRELUDE_PATH,
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();
    let mut pipeline_state: Option<RenderPipelineState> = None;
    let start_time = std::time::Instant::now();

    events_loop.run(move |event, _, control_flow| {
        autoreleasepool(|| {
            let res = (|| -> Result<(), Box<dyn std::error::Error>> {
                *control_flow = ControlFlow::Poll;

                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(size) => {
                            layer.set_drawable_size(CGSize::new(
                                size.width as f64,
                                size.height as f64,
                            ));
                        }
                        _ => (),
                    },
                    Event::MainEventsCleared => {
                        window.request_redraw();
                    }
                    Event::RedrawRequested(_) => {
                        let file_event = rx.try_recv();
                        if pipeline_state.is_none() && run_error.is_none() || file_event.is_ok() {
                            let library = shader_compiler::compile_shader(
                                &device,
                                |fragment_shader_in_msl| {
                                    println!("{}", fragment_shader_in_msl);
                                },
                            )?;
                            pipeline_state = Some(prepare_pipeline_state(
                                &device,
                                &library,
                                "clear_rect_vertex",
                                "clear_rect_fragment",
                            )?);
                        }
                        if pipeline_state.is_none() {
                            return Ok(());
                        }
                        // let size_for_shader_buffer = device.new_buffer_with_data(
                        //     vec![physical_size.width as u32, physical_size.height as u32].as_ptr()
                        //         as *const _,
                        //     mem::size_of::<u32>() as u64,
                        //     MTLResourceOptions::CPUCacheModeDefaultCache
                        //         | MTLResourceOptions::StorageModeManaged,
                        // );

                        let render_pass_descriptor = RenderPassDescriptor::new();

                        let drawable = layer.next_drawable().ok_or("No drawable")?;
                        prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

                        let command_buffer = command_queue.new_command_buffer();
                        let encoder =
                            command_buffer.new_render_command_encoder(&render_pass_descriptor);

                        let physical_size = window.inner_size();
                        encoder.set_scissor_rect(MTLScissorRect {
                            x: 0,
                            y: 0,
                            width: physical_size.width as _,
                            height: physical_size.height as _,
                        });

                        encoder.set_render_pipeline_state(pipeline_state.as_ref().unwrap());
                        encoder.set_vertex_buffer(0, Some(&vector_buffer), 0);
                        encoder.set_fragment_bytes(
                            0,
                            std::mem::size_of::<Input>() as u64,
                            &Input {
                                width: physical_size.width,
                                height: physical_size.height,
                                elapsed_time_secs: start_time.elapsed().as_secs_f32(),
                            } as *const Input as *const _,
                        );
                        encoder.draw_primitives_instanced(
                            metal::MTLPrimitiveType::TriangleStrip,
                            0,
                            4,
                            1,
                        );
                        // encoder.set_fragment_buffer(0, Some(&size_for_shader_buffer), 0);

                        encoder.end_encoding();
                        command_buffer.present_drawable(&drawable);
                        command_buffer.commit();
                    }
                    _ => {}
                };
                Ok(())
            })();
            if let Err(err) = res {
                println!("{}", err);
                run_error = Some(());
            }
        });
    });
    // Validate API
    #[allow(unreachable_code)]
    {
        let _ = shader::pixel_color(
            msl_prelude::Float2 { x: 0.0, y: 0.0 },
            msl_prelude::PixelInput {
                window_size: msl_prelude::Float2 { x: 0.0, y: 0.0 },
                elapsed_time_secs: 23.23,
            },
        );
        Ok(())
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
