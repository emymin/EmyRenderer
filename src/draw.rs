use pixels::{Pixels, SurfaceTexture};
use winit::{window::Window};

pub struct FrameBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
}

pub struct Float4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub fn new_frame_buffer(width: u32, height: u32, window:&Window) -> Result<FrameBuffer, String> {
    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height,&window);
    if let Ok(pixels) = 
        Pixels::new(width,height,surface_texture)
    {
        Ok(FrameBuffer{
            width,
            height,
            pixels,
        })
    } else {
        return Err("Failed to initialize frame buffer".to_string());
    }
}

pub fn linear_to_byte(value: f32) -> u8 {
    (value * 255.0) as u8
}

pub fn set_pixel(buffer:&mut FrameBuffer,x:u32,y:u32,color:&Float4){
    let frame = buffer.pixels.get_frame(); 
    let index:usize = ((y*buffer.width+x)*4) as usize;
    frame[index] = linear_to_byte(color.x);
    frame[index+1] = linear_to_byte(color.y);
    frame[index+2] = linear_to_byte(color.z);
    frame[index+3] = linear_to_byte(color.w);
}

pub fn clear_frame(buffer:&mut FrameBuffer){
    let frame = buffer.pixels.get_frame();
    for i in 0..frame.len(){
        frame[i] = 0;
    }
}

pub fn line(x0:f32,y0:f32,x1:f32,y1:f32,color:&Float4,buffer:&mut FrameBuffer){
    let mut t=0.0;
    while t<1.0{
        let x = x0 + (x1-x0)*t;
        let y = y0 + (y1-y0)*t;
        set_pixel(buffer,x as u32,y as u32,color);
        t+=0.1;
    }
}