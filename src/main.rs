#![forbid(unsafe_code)]
#![allow(dead_code)]

use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize
};
use winit_input_helper::WinitInputHelper;
use std::time::Instant;


pub mod draw;
pub mod model;
pub mod shader;

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

    let light = shader::Light{
        position: glam::Vec3::new(-1.0, -1.0, 2.0),
        color: glam::Vec3::new(1.0, 1.0, 1.0),
        intensity: 1.0,
    };

    let shader = shader::Shader{
        main_texture: shader::Texture::load("/dev/assets/bunnytex.jpg").expect("Failed to load texture"),
        lights: vec![light],
    };

    let models = model::load_obj("/dev/assets/bunny.obj").expect("Failed to load model");

    for model in models.iter(){
        let now = Instant::now();
        canvas.draw_model(&model,&shader,false);
        let elapsed = now.elapsed();
        println!("{} drawn in {} ms",model.name,elapsed.as_millis());
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
