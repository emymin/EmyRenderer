#![forbid(unsafe_code)]

use log::{error};
use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize
};
use winit_input_helper::WinitInputHelper;
mod draw;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Emy Renderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut frame_buffer = draw::new_frame_buffer(WIDTH, HEIGHT, &window).expect("There was an error creating the frame buffer");
    draw::clear_frame(&mut frame_buffer);


    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if frame_buffer.pixels
            .render()
            .map_err(|e| error!("pixels.render() failed: {}", e))
            .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event){
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                println!("Exiting...");
                return;
            }
        }
        let color = draw::Float4{x:1.0,y:0.0,z:0.0,w:1.0};

        draw::line(0.0, 0.0, 10.0, 10.0, &color, &mut frame_buffer);
        draw::set_pixel(&mut frame_buffer, 0, 0, &color);
        
        

    })

}
