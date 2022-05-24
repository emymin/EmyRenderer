#![forbid(unsafe_code)]
#![allow(dead_code)]

use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize
};
use winit_input_helper::WinitInputHelper;
pub mod draw;
pub mod model;

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

    let mut canvas = draw::Canvas::new(WIDTH, HEIGHT, &window).expect("There was an error creating the frame buffer");
    canvas.clear_frame();

    let models = model::Model::load_obj("/dev/assets/bunny.obj").expect("Failed to load model");
    for model in models.iter(){
        canvas.draw_wireframe(&model);
    }

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            canvas.render();
        }

        if input.update(&event){
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                println!("Exiting...");
                return;
            }
        }
        
        

    })

}
