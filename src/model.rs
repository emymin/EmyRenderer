use crate::shader::Texture;
use std::path;

pub struct Material{
    pub albedo_texture: Texture,
}

pub struct Vertex{
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2,
}
pub struct Face{
    pub vertices: [usize; 3],
    pub normal: glam::Vec3,
}

pub struct Model {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub material:Material
}

pub fn load_obj(path: &str) -> Result<Vec<Model>,String>{
    //get path of the directory
    let directory = path::Path::new(path).parent().unwrap();

    let mut loaded_models = Vec::<Model>::new();

    let(models,materials) = 
        tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
        .expect("Failed to OBJ load file");

    let materials = match materials{
        Ok(materials) => materials,
        Err(e) => {println!("No materials found, {}",e);vec![]},
    };

    for model in models.iter(){
        
        let mut vertices = Vec::<Vertex>::new();
        let mut faces = Vec::<Face>::new();

        let mut material = Material{
            albedo_texture: Texture::white(),
        };
        if materials.len()>0{
            let obj_material = &materials[model.mesh.material_id.unwrap()];
            let albedo_texture = &obj_material.diffuse_texture;
            if albedo_texture.len()>0 {
                let texture_path = directory.join(albedo_texture);
                material.albedo_texture = Texture::load(&texture_path.to_str().unwrap()).unwrap();
            }
        }

        for i in 0..model.mesh.positions.len()/3{
            let position:glam::Vec3 = glam::Vec3::new(
                model.mesh.positions[i*3],
                model.mesh.positions[i*3+1],
                model.mesh.positions[i*3+2],
            );
            let normal:glam::Vec3 = glam::Vec3::new(
                model.mesh.normals[i*3],
                model.mesh.normals[i*3+1],
                model.mesh.normals[i*3+2],
            );
            let uv:glam::Vec2 = glam::Vec2::new(
                model.mesh.texcoords[i*2],
                1.0-model.mesh.texcoords[i*2+1],
            );
            vertices.push(Vertex{
                position,
                normal,
                uv,
            });
        }

        for i in 0..model.mesh.indices.len()/3{
            let face_vertices = [
                model.mesh.indices[i*3] as usize,
                model.mesh.indices[i*3+1] as usize,
                model.mesh.indices[i*3+2] as usize,
            ];
            //(model.vertices[face.vertices[0]].position.xyz()-model.vertices[face.vertices[1]].position.xyz()).cross(model.vertices[face.vertices[1]].position.xyz()-model.vertices[face.vertices[2]].position.xyz()).normalize();
            let face_normal = (vertices[face_vertices[2]].position-vertices[face_vertices[0]].position)
                                    .cross(vertices[face_vertices[1]].position-vertices[face_vertices[0]].position)
                                    .normalize();
            let face = Face{
                vertices:face_vertices,
                normal:face_normal,
                
            };
            faces.push(face);
        }

        loaded_models.push(Model{
            name:model.name.clone(),
            vertices: vertices,
            faces: faces,
            material: material,
        });
    };

    Ok(loaded_models)

}
