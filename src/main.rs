use std::num::NonZeroU32;
use pangocairo::cairo;
use pangocairo::cairo::Format;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoopBuilder};
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoopBuilder::<()>::with_user_event().build();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let mut size = PhysicalSize::default();

    event_loop.run(move |event, _, control_flow| {
        use winit::event::Event::*;
        use winit::event::WindowEvent::CloseRequested;

        *control_flow = ControlFlow::Wait;

        match event {
            WindowEvent { window_id, event: CloseRequested }
                if window_id == window.id() => {
                    *control_flow = ControlFlow::Exit;
                }

            UserEvent(_) => {
                window.request_redraw();
            }

            RedrawRequested(window_id)
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

                    let mut cr = cairo::Context::new(&image_surface).unwrap();
                    // todo: draw over cr
                    buffer.present().unwrap();
                }

            // NewEvents(_) => {}
            // WindowEvent { window_id: _, event: _ } => {}
            // Suspended => {}
            // Resumed => {}
            // MainEventsCleared => {}
            // RedrawRequested(_) => {}
            // RedrawEventsCleared => {}
            // LoopDestroyed => {}
            _ => ()
        }
    });
}

