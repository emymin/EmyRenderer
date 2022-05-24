use pixels::{Pixels, SurfaceTexture};
use winit::{window::Window};
use std::mem;
use crate::model::{Model};

pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
}

fn linear_to_byte(value: f32) -> u8 {
    (value * 255.0) as u8
}

impl Canvas {

    pub fn new(width: u32, height: u32, window:&Window) -> Result<Canvas, String> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height,&window);
        if let Ok(pixels) = 
            Pixels::new(width,height,surface_texture)
        {
            Ok(Canvas{
                width,
                height,
                pixels,
            })
        } else {
            return Err("Failed to initialize frame buffer".to_string());
        }
    }

    pub fn render(&self){
        self.pixels.render().unwrap();
    }
    
    
    pub fn set_pixel(&mut self,x:i32,y:i32,color:&glam::Vec4){
        let frame = self.pixels.get_frame(); 
        if x>=self.width as i32 || y>=self.height as i32 || x<0 || y<0 {
            return;
        }
        let index = ((y as u32*self.width+x as u32)*4) as usize;
        frame[index] = linear_to_byte(color.x);
        frame[index+1] = linear_to_byte(color.y);
        frame[index+2] = linear_to_byte(color.z);
        frame[index+3] = linear_to_byte(color.w);
    }
    
    pub fn clear_frame(&mut self){
        let frame = self.pixels.get_frame();
        for i in 0..frame.len(){
            frame[i] = 0;
        }
    }
    
    pub fn draw_line(&mut self,x0:i32,y0:i32,x1:i32,y1:i32,color:&glam::Vec4){
        let mut steep = false;
        let (mut x0,mut y0,mut x1,mut y1) = (x0,y0,x1,y1);
        if (x0-x1).abs()<(y0-y1).abs(){
            steep = true;
            mem::swap(&mut x0,&mut y0);
            mem::swap(&mut x1,&mut y1);
        }
        if x0>x1{
            mem::swap(&mut x0,&mut x1);
            mem::swap(&mut y0,&mut y1);
        }
    
        let dx = x1-x0;
        let dy = y1-y0;
        let derror_double = dy.abs()*2;
        let mut error_double = 0;
        let mut y = y0;
        let mut x = x0;
    
        while x<x1{
            if steep{
                self.set_pixel(y,x,color);
            } else {
                self.set_pixel(x,y,color);
            }
            error_double += derror_double;
            if error_double>dx{
                y += if y1>y0{1}else{-1};
                error_double -= dx*2;
            }
    
            x+=1;
        }
    }
    
    pub fn draw_wireframe(&mut self,model:&Model){
        let width = self.width as f32;
        let height = self.height as f32;
        for i in (0..model.indices.len()).step_by(3){
            for j in 0..3{
                let i1 = model.indices[i+j] as usize;
                let i2 = model.indices[i+((j+1)%3)] as usize;

                let x0 = (model.vertices[i1*3] + 1.0) * width / 2.0;
                let y0 = (model.vertices[i1*3+1]*-1. + 1.0) * height / 2.0;
                let x1 = (model.vertices[i2*3] + 1.0) * width / 2.0;
                let y1 = (model.vertices[i2*3+1]*-1. + 1.0) * height / 2.0;

                self.draw_line(
                    x0 as i32,
                    y0 as i32,
                    x1 as i32,
                    y1 as i32,
                    &glam::Vec4::new(1.0,1.0,1.0,1.0)
                );
            }
        }
    }
}
