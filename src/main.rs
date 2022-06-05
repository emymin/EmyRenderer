#![forbid(unsafe_code)]
#![allow(dead_code)]

use winit::{
    event::{Event,WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize
};
use winit_input_helper::WinitInputHelper;
use std::time::Instant;
use clap::{Arg, Command};


pub mod draw;
pub mod model;
pub mod shader;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {

    let matches = Command::new("EmyRenderer")
        .version("0.1.0")
        .author("emymin")
        .about("Renders all your models")
        .arg(Arg::new("Path")
                 .required(true)
                 .short('p')
                 .long("path")
                 .takes_value(true)
                 .help("The path of the model to render"))
        .arg(Arg::new("Use Wireframe")
                 .short('w')
                 .long("use_wireframe")
                 .takes_value(true)
                 .help("Draws the model in wireframe")
                 .default_value("false")
                )
        .get_matches();

    let path = matches.value_of("Path").unwrap_or("");
    let is_wireframe = matches.value_of("Use Wireframe").unwrap_or("false").parse::<bool>().unwrap();
 
    //Load models
    let models = model::load_obj(path).expect("Failed to load model");

    //Create window
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("EmyRenderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    let mut canvas = draw::Canvas::new(WIDTH, HEIGHT, &window).expect("There was an error creating the frame buffer");

    let light = shader::Light{
        position: glam::Vec3::new(-1.0, -1.0, 2.0),
        color: glam::Vec3::new(1.0, 1.0, 1.0),
        intensity: 1.0,
    };
    
    let mut shader = shader::Shader{
        lights: vec![light],
    };
    
    let time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                let t = time.elapsed().as_secs_f32();
                shader.lights[0].position = glam::Vec3::new(t.sin(),t.cos(),t.sin());
        
                let start = Instant::now();
                canvas.clear_frame();
                for model in models.iter(){
                    canvas.draw_model(&model,&shader,is_wireframe);
                }
                let elapsed = start.elapsed();
                window.set_title(&format!("EmyRenderer | Frame Time: {:?}", elapsed.as_millis()));
                canvas.render();
            },
            _ => ()
        }

        if input.update(&event){
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                println!("Exiting...");
                return;
            }
            
        }



    });

}
