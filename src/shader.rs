use crate::model::Material;

pub struct Light{
    pub position: glam::Vec3,
    pub color: glam::Vec3,
    pub intensity: f32,
}

pub struct Texture{
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

impl Texture{
    pub fn load(path: &str) -> Result<Texture, String> {
        println!("Loading texture from {}", path);

        let image = image::open(path).expect("Failed to load image");
        let width = image.width();
        let height = image.height();
        let pixels = image.to_rgba8().into_vec();

        Ok(Texture{
            width,
            height,
            pixels,
        })
    }

    pub fn white() -> Texture {
        Texture{
            width: 1,
            height: 1,
            pixels: vec![255, 255, 255, 255],
        }
    }

    pub fn get_color(&self,x: u32, y: u32) -> glam::Vec4{
        let index = (y * self.width + x) as usize;
        let r = self.pixels[index * 4 + 0] as f32 / 255.0;
        let g = self.pixels[index * 4 + 1] as f32 / 255.0;
        let b = self.pixels[index * 4 + 2] as f32 / 255.0;
        let a = self.pixels[index * 4 + 3] as f32 / 255.0;
        glam::Vec4::new(r,g,b,a)
    }

    pub fn get_color_uv(&self,uv:glam::Vec2) -> glam::Vec4{
        let x = (uv.x.fract() *self.width as f32) as u32;
        let y = (uv.y.fract() *self.height as f32) as u32;
        return self.get_color(x, y);
    }
}

pub struct Shader{
    pub lights: Vec<Light>,
    
}

impl Shader{
    pub fn fragment(&self,uv:glam::Vec2,normal:glam::Vec3,position:glam::Vec3,material:&Material) -> glam::Vec4{
        let mut color = material.albedo_texture.get_color_uv(uv);

        let mut light_color = glam::Vec3::new(0.0,0.0,0.0);
        for light in &self.lights{
            let dir = light.position - position;
            let distance = dir.length();
            let light_dir = dir.normalize();
            light_color += light.color * light_dir.dot(normal).max(0.0) * (light.intensity / distance*distance);
        }

        color = color * glam::Vec4::from((light_color,1.0));

        //color = glam::Vec4::new(uv.x,uv.y,0.0,1.0);
        return color;
    }
}