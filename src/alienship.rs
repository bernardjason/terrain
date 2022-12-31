use crate::gl;
use crate::gl_helper::model::Model;
use crate::gl_helper::instance_model::ModelInstance;
use cgmath::{Matrix4, Vector3, };

pub struct AlienShip {
    pub instance:ModelInstance,
}

impl AlienShip {

    pub fn new(gl: &gl::Gl,position:Vector3<f32>) -> AlienShip {

        let model = Model::new(gl,"resources/alienship/invade.obj","resources/alienship/invade.png");
        let mut instance = ModelInstance::new(gl,model,0.4, None);
        instance.matrix = instance.matrix * Matrix4::<f32>::from_translation(position);
        AlienShip{
            instance,
        }
    }


    pub(crate) fn render(&mut self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>, our_shader:u32) {
        unsafe {
            gl.UseProgram(our_shader);

            self.instance.render(gl,view,projection,our_shader,false);
        }

    }

}