pub struct Model {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Model{
    pub fn load_obj(path: &str) -> Result<Vec<Model>,String>{
        let mut loaded_models = Vec::<Model>::new();

        let(models,_materials) = 
            tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)
            .expect("Failed to OBJ load file");

        for model in models.iter(){
            loaded_models.push(Model{
                vertices: model.mesh.positions.clone(),
                indices: model.mesh.indices.clone(),
            });
        };

        Ok(loaded_models)

    }
}
