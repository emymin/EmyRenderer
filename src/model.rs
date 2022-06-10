use crate::shader::Texture;
use std::path;

pub struct Material{
    pub albedo_texture: Texture,
    pub normal_texture: Texture,
    pub specular_texture: Texture,
}

pub struct Vertex{
    pub position: glam::Vec3,
    pub uv: glam::Vec2,
    pub normal: glam::Vec3,
    pub tangent: glam::Vec3,
    pub bitangent: glam::Vec3,
}
pub struct Face{
    pub vertices: [usize; 3],
    pub normal: glam::Vec3,
    pub tangent: glam::Vec3,
    pub bitangent: glam::Vec3,
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
            normal_texture: Texture::normal_default(),
            specular_texture: Texture::black(),
        };
        if materials.len()>0{
            let obj_material = &materials[model.mesh.material_id.unwrap()];
            let albedo_texture = &obj_material.diffuse_texture;
            if albedo_texture.len()>0 {
                material.albedo_texture = Texture::load(&directory.join(albedo_texture).to_str().unwrap()).unwrap();
            }
            let normal_texture = &obj_material.normal_texture;
            if normal_texture.len()>0 {
                material.normal_texture = Texture::load(&directory.join(normal_texture).to_str().unwrap()).unwrap();
            }
            let specular_texture = &obj_material.specular_texture;
            if specular_texture.len()>0 {
                material.specular_texture = Texture::load(&directory.join(specular_texture).to_str().unwrap()).unwrap();
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
                model.mesh.texcoords[i*2+1],
            );
            let tangent = glam::Vec3::ZERO; //tangent and bitangent are calculated while iterating faces
            let bitangent = glam::Vec3::ZERO;

            vertices.push(Vertex{
                position:position,
                uv:uv,
                normal:normal,
                tangent:tangent,
                bitangent:bitangent,
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
            
            let deltapos1 = vertices[face_vertices[1]].position-vertices[face_vertices[0]].position;
            let deltapos2 = vertices[face_vertices[2]].position-vertices[face_vertices[1]].position;
            let deltauv1 = vertices[face_vertices[1]].uv-vertices[face_vertices[0]].uv;
            let deltauv2 = vertices[face_vertices[2]].uv-vertices[face_vertices[1]].uv;

            let r = 1.0/(deltauv1.x*deltauv2.y-deltauv1.y*deltauv2.x);
            let tangent = (deltapos1*deltauv2.y-deltapos2*deltauv1.y)*r;
            let bitangent = (deltapos2*deltauv1.x-deltapos1*deltauv2.x)*r;

            for v in face_vertices.iter(){
                vertices[*v].tangent = tangent;
                vertices[*v].bitangent = bitangent;
            }

            let face = Face{
                vertices:face_vertices,
                normal:face_normal,
                tangent:tangent,
                bitangent:bitangent,
                
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
