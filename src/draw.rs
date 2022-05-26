use pixels::{Pixels, SurfaceTexture};
use winit::{window::Window};
use std::mem;
use glam::Vec3Swizzles;
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

    pub fn draw_line_vec(&mut self,start:&glam::Vec2,end:&glam::Vec2,color:&glam::Vec4){
        self.draw_line(start.x as i32,start.y as i32,end.x as i32,end.y as i32,color);
    }
    
    pub fn draw_wire_triangle(&mut self, t0:glam::Vec2,t1:glam::Vec2,t2:glam::Vec2,color:&glam::Vec4){
        self.draw_line_vec(&t0,&t1,color);
        self.draw_line_vec(&t1,&t2,color);
        self.draw_line_vec(&t2,&t0,color);
    }
    

    pub fn draw_triangle(&mut self, t0:glam::Vec2,t1:glam::Vec2,t2:glam::Vec2,color:&glam::Vec4){
        let (mut t0,mut t1,mut t2) = (t0,t1,t2);
        
        if t0.y==t1.y && t0.y==t2.y{return;}
        if t0.y>t1.y {mem::swap(&mut t0,&mut t1);}
        if t0.y>t2.y {mem::swap(&mut t0,&mut t2);}
        if t1.y>t2.y {mem::swap(&mut t1,&mut t2);}
        let total_height = t2.y-t0.y;
        let mut i = 0f32;
        while i<total_height {
            let second_half = i>t1.y-t0.y || t1.y==t0.y;
            let segment_height = if second_half{t2.y-t1.y}else{t1.y-t0.y};
            
            let alpha = i/total_height;
            let beta = (i-if second_half{t1.y-t0.y}else{0.0})/segment_height;
            
            let mut a = t0 + (t2-t0)*alpha;
            let mut b = if second_half{t1+(t2-t1)*beta}else{t0+(t1-t0)*beta};

            if a.x>b.x{
                mem::swap(&mut a,&mut b);
            }
            let mut j = a.x as i32;
            while j<b.x as i32{
                self.set_pixel(j, (t0.y as i32) + (i as i32), color);
                j+=1;
            }
            i+=1.0;
        }

    }

    pub fn draw_model(&mut self,model:&Model,is_wireframe:bool){
        let width = self.width as f32;
        let height = self.height as f32;
        let scale_vec = glam::Vec2::new(width/2.0,height/2.0);
        let flip_vec = glam::Vec2::new(1.0,-1.0);
        for (i,face) in model.faces.iter().enumerate(){
            let t0:glam::Vec2 = (model.vertices[face.vertices[0]].position.xy()*flip_vec + 1.0)*scale_vec;
            let t1:glam::Vec2 = (model.vertices[face.vertices[1]].position.xy()*flip_vec + 1.0)*scale_vec;
            let t2:glam::Vec2 = (model.vertices[face.vertices[2]].position.xy()*flip_vec + 1.0)*scale_vec;
            if is_wireframe{
                self.draw_wire_triangle(t0,t1,t2,&glam::Vec4::ONE);
            } else {
                let light_dir = glam::Vec3::new(0.0,0.0,-1.0);
                let n = (model.vertices[face.vertices[0]].position.xyz()-model.vertices[face.vertices[1]].position.xyz()).cross(model.vertices[face.vertices[1]].position.xyz()-model.vertices[face.vertices[2]].position.xyz()).normalize();
                let intensity = n.dot(light_dir);
                
                self.draw_triangle(t0,t1,t2,&(glam::Vec4::ONE*intensity));
            }
            //println!("{}%",(i as f32)/(model.faces.len() as f32)*100.0);
        }
    }


}
