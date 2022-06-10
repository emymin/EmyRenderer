pub fn viewport_matrix(x:f32,y:f32,width:f32,height:f32,depth:f32) -> glam::Mat4{
    let m = glam::Mat4::from_cols_array(&[
        width/2.0, 0.0, 0.0, 0.0,
        0.0, height/2.0, 0.0, 0.0,
        0.0, 0.0, depth/2.0, 0.0,
        x+width/2.0, y+height/2.0, depth/2.0, 1.0,
    ]);
    return m;
}


pub struct Camera{
    pub view:glam::Mat4,
    pub projection:glam::Mat4,
    pub viewport:glam::Mat4,
    pub position:glam::Vec3,
}

impl Camera{
    pub fn new(width:u32,height:u32) -> Camera{
        let position = glam::Vec3::new(0.0,0.0,1.0);
        let view = glam::Mat4::look_at_rh(position,glam::Vec3::ZERO,glam::Vec3::new(0.0,1.0,0.0));
        let projection = glam::Mat4::perspective_lh(f32::to_radians(60.0),
            width as f32/height as f32,
            0.1,
            100.0);
        let viewport = viewport_matrix(0.0,0.0,width as f32,height as f32,1.0);

        Camera{
            view,
            projection,
            viewport,
            position,
        }
    }

    pub fn look_at(&mut self,eye:glam::Vec3,target:glam::Vec3,up:glam::Vec3){
        self.view = glam::Mat4::look_at_rh(eye,target,up);
        self.position = eye;
    }
}