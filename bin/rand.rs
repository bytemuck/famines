use emu6502::Processor;
use pixels::{Error, Pixels, SurfaceTexture};
use rand::prelude::*;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 64;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64 * 8f64, HEIGHT as f64 * 8f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    let mut processor = Processor::new();
    processor.registers.pc = 0x0200; // starts at 0x0200

    let mut rng = rand::thread_rng();

    let mut x: u32 = 0;
    for i in 0xA000..0xB000 {
        processor.memory[i as u16] = (x % 256) as u8;
        x += 1;
    }

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if !(input.state == ElementState::Pressed) {
                        return;
                    };

                    if let Some(kc) = input.virtual_keycode {
                        match kc {
                            VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                            VirtualKeyCode::Space => {
                                for i in 0xA000..0xB000 {
                                    processor.memory[i as u16] = rng.gen();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            },
            Event::RedrawRequested(_) => {
                let frame = pixels.get_frame();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let color = processor.memory[(0xA000 + i) as u16];

                    let red = color & 0b11_00_00_00;
                    let green = (color & 0b00_11_00_00) << 2;
                    let blue = (color & 0b00_00_11_00) << 4;
                    let aplha = (color & 0b_00_00_00_11) << 6;

                    let rgba = [red, green, blue, aplha];

                    pixel.copy_from_slice(&rgba);
                }

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => {}
        }

        window.request_redraw();
    });
}
