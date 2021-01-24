// Copyright 2016 metal-rs developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate objc;

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
struct Size {
    pub width: u32,
    pub height: u32,
}

fn main() {
    let events_loop = winit::event_loop::EventLoop::new();
    let size = winit::dpi::LogicalSize::new(800, 600);

    let window = winit::window::WindowBuilder::new()
        .with_inner_size(size)
        .with_title("Metal Window Example")
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

    let mut r = 0.0f32;

    let clear_rect = vec![ClearRect {
        rect: Rect {
            x: -1.0,
            y: -1.0,
            w: 2.0,
            h: 2.0,
        },
        color: Color {
            r: 0.5,
            g: 0.8,
            b: 0.5,
            a: 1.0,
        },
    }];

    let clear_rect_buffer = device.new_buffer_with_data(
        clear_rect.as_ptr() as *const _,
        mem::size_of::<ClearRect>() as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
    );

    let command_queue = device.new_command_queue();
    let mut compiled: Option<String> = None;
    let mut run_error: Option<String> = None;

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
                        let library =
                            shader_compiler::compile_shader(&device, |fragment_shader_in_msl| {
                                if compiled.as_ref() != Some(&fragment_shader_in_msl) {
                                    println!("{}", fragment_shader_in_msl);
                                    compiled = Some(fragment_shader_in_msl);
                                }
                            })?;

                        let drawable = layer.next_drawable().ok_or("No drawable")?;
                        let clear_rect_pipeline_state = prepare_pipeline_state(
                            &device,
                            &library,
                            "clear_rect_vertex",
                            "clear_rect_fragment",
                        )?;

                        let physical_size = window.inner_size();
                        // let size_for_shader_buffer = device.new_buffer_with_data(
                        //     vec![physical_size.width as u32, physical_size.height as u32].as_ptr()
                        //         as *const _,
                        //     mem::size_of::<u32>() as u64,
                        //     MTLResourceOptions::CPUCacheModeDefaultCache
                        //         | MTLResourceOptions::StorageModeManaged,
                        // );

                        let render_pass_descriptor = RenderPassDescriptor::new();

                        prepare_render_pass_descriptor(&render_pass_descriptor, drawable.texture());

                        let command_buffer = command_queue.new_command_buffer();
                        let encoder =
                            command_buffer.new_render_command_encoder(&render_pass_descriptor);

                        encoder.set_scissor_rect(MTLScissorRect {
                            x: 0,
                            y: 0,
                            width: physical_size.width as _,
                            height: physical_size.height as _,
                        });

                        encoder.set_render_pipeline_state(&clear_rect_pipeline_state);
                        encoder.set_vertex_buffer(0, Some(&clear_rect_buffer), 0);
                        encoder.set_fragment_bytes(
                            0,
                            std::mem::size_of::<Size>() as u64,
                            &Size {
                                width: physical_size.width,
                                height: physical_size.height,
                            } as *const Size as *const _,
                        );
                        encoder.draw_primitives_instanced(
                            metal::MTLPrimitiveType::TriangleStrip,
                            0,
                            4,
                            1,
                        );
                        // encoder.set_fragment_buffer(0, Some(&size_for_shader_buffer), 0);

                        // encoder.set_scissor_rect(MTLScissorRect {
                        //     x: 0,
                        //     y: 0,
                        //     width: physical_size.width as _,
                        //     height: physical_size.height as _,
                        // });

                        // encoder.set_render_pipeline_state(&triangle_pipeline_state);
                        // encoder.set_vertex_buffer(0, Some(&vbuf), 0);
                        // encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
                        encoder.end_encoding();

                        command_buffer.present_drawable(&drawable);
                        command_buffer.commit();

                        r += 0.01f32;
                    }
                    _ => {}
                };
                Ok(())
            })();
            if let Err(err) = res {
                let message = format!("{}", err);
                if run_error != Some(message.clone()) {
                    println!("{}", message);
                }
                run_error = Some(message);
            }
        });
    });
    // Validate API
    #[allow(unreachable_code)]
    {
        let _ = shader::pixel_color(
            msl_prelude::Float2 { x: 0.0, y: 0.0 },
            msl_prelude::Float2 { x: 0.0, y: 0.0 },
        );
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

    // attachment.set_blending_enabled(true);
    // attachment.set_rgb_blend_operation(metal::MTLBlendOperation::Add);
    // attachment.set_alpha_blend_operation(metal::MTLBlendOperation::Add);
    // attachment.set_source_rgb_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    // attachment.set_source_alpha_blend_factor(metal::MTLBlendFactor::SourceAlpha);
    // attachment.set_destination_rgb_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    // attachment.set_destination_alpha_blend_factor(metal::MTLBlendFactor::OneMinusSourceAlpha);
    Ok(device.new_render_pipeline_state(&pipeline_state_descriptor)?)
}

fn prepare_render_pass_descriptor(descriptor: &RenderPassDescriptorRef, texture: &TextureRef) {
    //descriptor.color_attachments().set_object_at(0, MTLRenderPassColorAttachmentDescriptor::alloc());
    //let color_attachment: MTLRenderPassColorAttachmentDescriptor = unsafe { msg_send![descriptor.color_attachments().0, _descriptorAtIndex:0] };//descriptor.color_attachments().object_at(0);
    let color_attachment = descriptor.color_attachments().object_at(0).unwrap();

    color_attachment.set_texture(Some(texture));
    color_attachment.set_load_action(MTLLoadAction::Clear);
    color_attachment.set_clear_color(MTLClearColor::new(0.2, 0.2, 0.25, 1.0));
    color_attachment.set_store_action(MTLStoreAction::Store);
}