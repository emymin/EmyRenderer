#![forbid(unsafe_code)]

use log::{debug, error};
use winit::{
    event::{Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::LogicalSize
};
use winit_input_helper::WinitInputHelper;
use pixels::{Error, Pixels, SurfaceTexture};


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

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height,&window);
        Pixels::new(WIDTH,HEIGHT,surface_texture).expect("Failed to initialize pixels surface")
    };

    let frame = pixels.get_frame();
    clear_frame(frame);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            if pixels
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
                return;
            }
        }

        let frame = pixels.get_frame();
        

    })

}

fn set_pixel(frame:&mut [u8],x:u32,y:u32,r:u8,g:u8,b:u8,a:u8){
    let index:usize = ((y*WIDTH+x)*4) as usize;
    frame[index] = r;
    frame[index+1] = g;
    frame[index+2] = b;
    frame[index+3] = a;
}

fn clear_frame(frame:&mut [u8]){
    for i in 0..frame.len(){
        frame[i] = 0;
    }
}