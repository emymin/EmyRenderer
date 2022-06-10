use glam::Vec4Swizzles;

use crate::model::{Material,Vertex};
use crate::draw::{interpolate_bc};
use crate::camera::{Camera};


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

    pub fn black() -> Texture {
        Texture{
            width: 1,
            height: 1,
            pixels: vec![0, 0, 0, 255],
        }
    }

    pub fn normal_default() -> Texture {
        Texture{
            width: 1,
            height: 1,
            pixels: vec![128, 128, 255, 255],
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
        let x = (uv.x.fract() * (self.width-1) as f32) as u32;
        let y = (((1.0-uv.y).fract()) * (self.height-1) as f32) as u32;
        return self.get_color(x,y);
    }
}

pub struct GlobalData{
    pub ambient_light: glam::Vec3,
    pub lights: Vec<Light>,
    pub time: f32,
    pub camera: Camera
}
pub struct VertInput{
    pub mvpv: glam::Mat4,
    pub mvp : glam::Mat4,
    pub mv : glam::Mat4,
    pub m: glam::Mat4,
    pub mit: glam::Mat4,
}
pub struct VertOutput{
    pub position : glam::Vec3,
    pub world_position: glam::Vec3,
    pub uv : glam::Vec2,
    pub normal : glam::Vec3,
    pub tangent : glam::Vec3,
    pub bitangent : glam::Vec3,
}

pub fn interpolate_vertoutput(a:&VertOutput,b:&VertOutput,c:&VertOutput,barycentric:&glam::Vec3) -> VertOutput{
    let position = interpolate_bc(a.position,b.position,c.position,barycentric);
    let world_position = interpolate_bc(a.world_position,b.world_position,c.world_position,barycentric);
    let uv = interpolate_bc(a.uv,b.uv,c.uv,barycentric);
    let normal = interpolate_bc(a.normal,b.normal,c.normal,barycentric);
    let tangent = interpolate_bc(a.tangent,b.tangent,c.tangent,barycentric);
    let bitangent = interpolate_bc(a.bitangent,b.bitangent,c.bitangent,barycentric);
    VertOutput{
        position:position,
        world_position:world_position,
        uv:uv,
        normal:normal,
        tangent:tangent,
        bitangent:bitangent
    }
}
pub fn reflect(normal:glam::Vec3,direction:glam::Vec3) -> glam::Vec3{
    return direction - 2.0 * normal * (normal.dot(direction));
}

pub trait Shader{
    fn vertex(&self,vertex:&Vertex,i:&VertInput,globals:&GlobalData) -> VertOutput;
    fn fragment(&self,i:&VertOutput,material:&Material,globals:&GlobalData) -> glam::Vec4;
}

pub fn generic_vertex(vertex:&Vertex,i:&VertInput) -> VertOutput{
    let om = i.mvpv * glam::Vec4::from((vertex.position,1.0));
    let position = om.xyz()/om.w;
    let world_position = i.m * glam::Vec4::from((vertex.position,1.0));
    let normal = i.mit * glam::Vec4::from((vertex.normal,0.0));
    let tangent = i.mit * glam::Vec4::from((vertex.tangent,0.0));
    let bitangent = i.mit * glam::Vec4::from((vertex.bitangent,0.0));
    let uv = vertex.uv;
    VertOutput{
        position: position,
        world_position: world_position.xyz(),
        uv:uv,
        normal: normal.xyz(),
        tangent:tangent.xyz(),
        bitangent:bitangent.xyz(),
    }
}


pub struct LitShader{}
impl Shader for LitShader{
    fn fragment(&self,i:&VertOutput,material:&Material,globals:&GlobalData) -> glam::Vec4{
        let tbn = glam::Mat3::from_cols(i.tangent.normalize(), i.bitangent.normalize(), i.normal.normalize());
        let normal_map = material.normal_texture.get_color_uv(i.uv);
        let normal = (normal_map.xyz() * 2.0 - 1.0).normalize();
        let normal = (tbn * normal).normalize();

        let albedo_texture = material.albedo_texture.get_color_uv(i.uv);
        let mut color = albedo_texture.xyz();
        let alpha = albedo_texture.w;
        let viewdir = (globals.camera.position-i.world_position).normalize();

        let mut light_color = glam::Vec3::new(0.0,0.0,0.0);
        let mut specular_color = glam::Vec3::new(0.0,0.0,0.0);

        let specular_power = material.specular_texture.get_color_uv(i.uv).length() * 256.0;
        

        for light in &globals.lights{
            let dir = light.position - i.world_position;
            let distance = dir.length();
            let light_dir = dir.normalize();

            let r = reflect(normal,-light_dir);
            let spec = r.dot(viewdir).max(0.0).powf(specular_power);
            specular_color += spec * light.color * light.intensity;

            specular_color += spec * light.color;
            light_color += light.color * light_dir.dot(normal).max(0.0) * (light.intensity / distance*distance);
        }

        color = globals.ambient_light + 
                color * 
                (light_color+specular_color);
        return glam::Vec4::from((color,alpha));
    }
    fn vertex(&self,vertex:&Vertex,i:&VertInput,_globals:&GlobalData) -> VertOutput{
        return generic_vertex(vertex, i);
    }
}

pub struct UnlitShader{}
impl Shader for UnlitShader{
    fn fragment(&self,i:&VertOutput,material:&Material,_globals:&GlobalData) -> glam::Vec4{
        return material.albedo_texture.get_color_uv(i.uv);
    }
    fn vertex(&self,vertex:&Vertex,i:&VertInput,_globals:&GlobalData) -> VertOutput{
        return generic_vertex(vertex,i);
    }
}

pub enum DebugMode{
    Uv,
    Normal,
    Position,
    Tangent,
    Bitangent,
    AlbedoMap,
    SpecularMap,
    NormalMap,
}

pub struct DebugShader{
    pub mode: DebugMode,
}
impl Shader for DebugShader{
    fn fragment(&self,i:&VertOutput,material:&Material,_globals:&GlobalData) -> glam::Vec4{
        match self.mode{
            DebugMode::Uv => return glam::Vec4::new(i.uv.x,i.uv.y,0.0,1.0),
            DebugMode::Normal => return glam::Vec4::from((i.normal,1.0)),
            DebugMode::Position => return glam::Vec4::from((i.world_position,1.0)),
            DebugMode::Tangent => return glam::Vec4::from((i.tangent,1.0)),
            DebugMode::Bitangent => return glam::Vec4::from((i.bitangent,1.0)),
            DebugMode::AlbedoMap => return material.albedo_texture.get_color_uv(i.uv),
            DebugMode::SpecularMap => return material.specular_texture.get_color_uv(i.uv),
            DebugMode::NormalMap => return material.normal_texture.get_color_uv(i.uv),
        }
    }
    fn vertex(&self,vertex:&Vertex,i:&VertInput,_globals:&GlobalData) -> VertOutput{
        return generic_vertex(vertex,i);
    }
}