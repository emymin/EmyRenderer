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
pub mod camera;


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
                .long("use_wireframe")
                .takes_value(true)
                .help("Draws the model in wireframe")
                .default_value("false"))
        .arg(Arg::new("Width")
                .short('w')
                .long("width")
                .takes_value(true)
                .help("The width of the window")
                .default_value("1280"))
        .arg(Arg::new("Height")
                .short('h')
                .long("height")
                .takes_value(true)
                .help("The height of the window")
                .default_value("720"))
        .get_matches();

    let path = matches.value_of("Path").unwrap_or("");
    let is_wireframe = matches.value_of("Use Wireframe").unwrap_or("false").parse::<bool>().unwrap();
    let width = matches.value_of("Width").unwrap_or("1280").parse::<u32>().unwrap();
    let height = matches.value_of("Height").unwrap_or("720").parse::<u32>().unwrap();
 
    //Load models
    let models = model::load_obj(path).expect("Failed to load model");

    //Create window
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("EmyRenderer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap()
    };
    let mut canvas = draw::Canvas::new(width, height, &window).expect("There was an error creating the frame buffer");

    let light1 = shader::Light{
        position: glam::Vec3::new(-1.0, -1.0, 2.0),
        color: glam::Vec3::new(1.0, 1.0, 1.0),
        intensity: 1.0,
    };
    
    let mut globals = shader::GlobalData{
        ambient_light: glam::Vec3::new(0.1, 0.1, 0.1),
        lights: vec![light1],
        time:0.0,
        camera: camera::Camera::new(width,height),
    };

    let shader = shader::LitShader{};
    //let shader = shader::DebugShader{mode:shader::DebugMode::Bitangent};

    
    let time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                let t = time.elapsed().as_secs_f32();
                globals.time = t;

                let eye = glam::Vec3::new(globals.time.sin()*2.0,1.0,globals.time.cos()*2.0);              
                let center = glam::Vec3::new(0.0,0.0,0.0);
                let up = glam::Vec3::new(0.0,1.0,0.0);
                globals.camera.look_at(eye,center,up);

                //globals.camera.look_at(glam::Vec3::new(0.0,0.0,(globals.time*0.1).sin()*2.0), glam::Vec3::new(0.0,0.0,50.0), glam::Vec3::new(0.0,1.0,0.0));
                
                globals.lights[0].position = glam::Vec3::new(globals.time.sin()*2.0,globals.time.sin(),globals.time.cos()*2.0);
                

                let start = Instant::now();
                canvas.clear_frame();
                for model in models.iter(){
                    canvas.draw_model(&model,&shader,&globals,is_wireframe);
                }
                let elapsed = start.elapsed();
                window.set_title(&format!("EmyRenderer | Frame Time: {} | FPS: {}", elapsed.as_millis(), 1.0 / elapsed.as_secs_f32()));
                canvas.render();
            },
            _ => ()
        }

        if input.update(&event){
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            
        }



    });

}
