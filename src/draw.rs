use pixels::{Pixels, PixelsBuilder, SurfaceTexture};
use winit::{window::Window};
use std::mem;
use glam::Vec3Swizzles;
use crate::model::{Model,Vertex,Material};
use crate::shader::{Shader,FragInput,VertInput,GlobalData};


pub struct Canvas {
    pub width: u32,
    pub height: u32,
    pub pixels: Pixels,
    pub depth_buffer: Vec<f32>,
    viewport_matrix: glam::Mat4,
}

fn linear_to_byte(value: f32) -> u8 {
    (value * 255.0) as u8
}

fn to_barycentric(a: glam::Vec3, b: glam::Vec3, c: glam::Vec3, p: glam::Vec3) -> glam::Vec3 {
    let s1 = glam::Vec3::new(c.x - a.x, b.x - a.x, a.x - p.x);
    let s2 = glam::Vec3::new(c.y - a.y, b.y - a.y, a.y - p.y);

    let u = s1.cross(s2);
    if u.z.abs()>1e-2{
        return glam::Vec3::new(
            1.0 - (u.x + u.y) / u.z,
            u.y / u.z,
            u.x / u.z,
        );
    }else{
        return glam::Vec3::new(-1.0, 1.0, 1.0);
    }

}

pub fn viewport_matrix(x:f32,y:f32,width:f32,height:f32,depth:f32) -> glam::Mat4{
    let m = glam::Mat4::from_cols_array(&[
        width/2.0, 0.0, 0.0, 0.0,
        0.0, height/2.0, 0.0, 0.0,
        0.0, 0.0, depth/2.0, 0.0,
        x+width/2.0, y+height/2.0, depth/2.0, 1.0,
    ]);
    return m;
}


impl Canvas {

    pub fn new(width: u32, height: u32, window:&Window) -> Result<Canvas, String> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height,&window);
        let depth_buffer = vec![f32::NEG_INFINITY; (width * height) as usize];

        let viewport_matrix = viewport_matrix(0.0, 0.0, width as f32, height as f32, 1.0);

        if let Ok(pixels) = 
            PixelsBuilder::new(width,height,surface_texture).build()
        {
            Ok(Canvas{
                width,
                height,
                pixels,
                depth_buffer,
                viewport_matrix,
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

    pub fn set_pixel_depth(&mut self,x:i32,y:i32,depth:f32){
        if x>=self.width as i32 || y>=self.height as i32 || x<0 || y<0 {
            return;
        }
        let index = (y as u32*self.width+x as u32) as usize;
        self.depth_buffer[index] = depth;
    }

    pub fn get_pixel_depth(&self,x:i32,y:i32) -> f32{
        if x>=self.width as i32 || y>=self.height as i32 || x<0 || y<0 {
            return f32::INFINITY;
        }
        let index = (y as u32*self.width+x as u32) as usize;
        self.depth_buffer[index]
    }
    
    pub fn clear_frame(&mut self){
        let frame = self.pixels.get_frame();
        frame.iter_mut().for_each(|x| *x=128);
        self.depth_buffer.iter_mut().for_each(|x| *x=f32::NEG_INFINITY);
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

    pub fn to_screen_space(&self,v:&glam::Vec3) -> glam::Vec3{
        glam::Vec3::new(
            (v.x+1.0)*self.width as f32/2.0,
            (v.y*-1.0+1.0)*self.height as f32/2.0,
            v.z
        )
    }
    
    pub fn draw_wire_triangle(&mut self, t0:glam::Vec2,t1:glam::Vec2,t2:glam::Vec2,color:&glam::Vec4){
        self.draw_line_vec(&t0,&t1,color);
        self.draw_line_vec(&t1,&t2,color);
        self.draw_line_vec(&t2,&t0,color);
    }
    

    pub fn draw_triangle(&mut self, v0:&Vertex,v1:&Vertex,v2:&Vertex,shader:&dyn Shader,material:&Material,vert_input:&VertInput,globals:&GlobalData, is_wireframe:bool){ 
        let t0 = shader.vertex(v0,&vert_input,globals);
        let t1 = shader.vertex(v1,&vert_input,globals);
        let t2 = shader.vertex(v2,&vert_input,globals);

        if is_wireframe {
            self.draw_wire_triangle(
                t0.xy(),
                t1.xy(),
                t2.xy(),
                &glam::Vec4::ONE,
            );
            return;
        }

        let mut max_box = glam::Vec2::new(0.0,0.0);
        let mut min_box = glam::Vec2::new((self.width-1) as f32,(self.height-1) as f32);
        let clamp = min_box.clone();
        for v in [t0,t1,t2] {
            max_box.x = max_box.x.max(v.x).min(clamp.x);
            max_box.y = max_box.y.max(v.y).min(clamp.y);

            min_box.x = min_box.x.min(v.x).max(0.0);
            min_box.y = min_box.y.min(v.y).max(0.0);
        }

        let mut x = min_box.x.ceil() as i32;
        while x<max_box.x.ceil() as i32{
            let mut y = min_box.y.ceil() as i32;
            while y<max_box.y.ceil() as i32{
                let bc = to_barycentric(t0,t1,t2,glam::Vec3::new(x as f32,y as f32, 0.0));
                if bc.x>=0.0 && bc.y>=0.0 && bc.z>=0.0 {
                    let z = bc.x*t0.z + bc.y*t1.z + bc.z*t2.z;
                    if z>self.get_pixel_depth(x, y){
                        let fraginput = FragInput{
                            uv : bc.x*v0.uv + bc.y*v1.uv + bc.z*v2.uv,
                            normal : bc.x*v0.normal + bc.y*v1.normal + bc.z*v2.normal,
                            position : bc.x*v0.position + bc.y*v1.position + bc.z*v2.position
                        };
                        let color = shader.fragment(&fraginput,material,globals);
                        self.set_pixel(x,y,&color);
                        self.set_pixel_depth(x,y,z);
                    }
                }
                y+=1;
            }
            x+=1;
        }
        
    }

    pub fn draw_model(&mut self,model:&Model,shader:&dyn Shader,globals:&GlobalData,is_wireframe:bool){
        let eye = glam::Vec3::new(globals.time.sin()*2.0,globals.time.sin(),globals.time.cos()*2.0);
        let center = glam::Vec3::new(0.0,0.0,0.0);

        let view_matrix = glam::Mat4::look_at_rh(eye,center,glam::Vec3::new(0.0,1.0,0.0));
        let projection_matrix = glam::Mat4::perspective_lh(f32::to_radians(60.0),
                                                                self.width as f32/self.height as f32,
                                                                0.1,
                                                                100.0);

        let model_matrix = glam::Mat4::IDENTITY;
        let mvp = self.viewport_matrix*projection_matrix*view_matrix*model_matrix;

        let v_in = VertInput { mvp: mvp};

        for (_i,face) in model.faces.iter().enumerate(){
            self.draw_triangle(
                &model.vertices[face.vertices[0]],
                &model.vertices[face.vertices[1]],
                &model.vertices[face.vertices[2]],
                shader,
                &model.material,
                &v_in,
                globals,
                is_wireframe,
            );
            //print!("\r{}%",((_i as f32)/(model.faces.len() as f32)*100.0).round());
        }
    }


}
