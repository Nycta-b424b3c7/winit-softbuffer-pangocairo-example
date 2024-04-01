use std::num::NonZeroU32;

use pangocairo::cairo::Format;
use pangocairo::cairo;
use pangocairo::functions::{create_context, show_layout};
use pangocairo::pango;
use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let context = softbuffer::Context::new(&window).unwrap();
    let mut surface = softbuffer::Surface::new(&context, &window).unwrap();

    let mut clicks = 0;
    let mut size = PhysicalSize::default();

    event_loop.run(|event, event_loop| {
        use winit::event::Event::*;
        use winit::event::WindowEvent::CloseRequested;
        use winit::event::WindowEvent::MouseInput;
        use winit::event::WindowEvent::RedrawRequested;
        use winit::event::ElementState::Pressed;

        match event {
            WindowEvent { window_id, event: CloseRequested }
                if window_id == window.id() => {
                    event_loop.exit();
                }

            WindowEvent { window_id, event: MouseInput { state, .. } }
                if window_id == window.id() => {
                    if matches!(state, Pressed) {
                        clicks += 1;
                    }
                    window.request_redraw();
                }

            WindowEvent { window_id, event: RedrawRequested }
                if window_id == window.id() => {
                    let window_size = window.inner_size();
                    let PhysicalSize { width, height } = window_size;
                    if size != window_size {
                        size = window_size;
                        let PhysicalSize { width, height } = size;
                        let width = NonZeroU32::new(width).unwrap();
                        let height = NonZeroU32::new(height).unwrap();
                        surface.resize(width, height).unwrap();
                    }

                    let mut buffer = surface.buffer_mut().unwrap();

                    let image_surface = unsafe {
                        use cairo::ImageSurface;
                        let data = buffer.as_mut_ptr();
                        let format = Format::Rgb24;
                        let stride = format.stride_for_width(width).unwrap();
                        ImageSurface::create_for_data_unsafe(
                            data as *mut u8,
                            Format::Rgb24,
                            width as i32,
                            height as i32,
                            stride
                        )
                    }.unwrap();

                    { // Actual rendering
                        let mut cr = cairo::Context::new(&image_surface).unwrap();
                        cr.save().unwrap();
                        cr.set_source_rgb(1.0, 1.0, 1.0);
                        cr.fill().unwrap();
                        cr.paint().unwrap();
                        cr.restore().unwrap();

                        cr.set_source_rgb(0.0, 0.0, 0.0);
                        cr.save().unwrap();
                        let pango_context = create_context(&cr);
                        cr.move_to(50.0, 50.0);
                        let pango_layout = pango::Layout::new(&pango_context);
                        let s = format!("clicks so far: {}", clicks);
                        pango_layout.set_text(&s);
                        show_layout(&cr, &pango_layout);
                        cr.restore().unwrap();
                    }

                    buffer.present().unwrap();
                }

            _ => ()
        }
    }).unwrap();
}

