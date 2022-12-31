#![allow(dead_code)]
use crate::gl_helper::model::Model;
use cgmath::{Matrix4, vec3};
use crate::gl_helper::gl_matrix4;
use std::ptr;

use crate::gl;
//use crate::game::Render;
use crate::gl_helper::texture::create_texture;

#[derive(Clone)]
pub struct ModelInstance {
    model: Model,
    pub(crate) matrix: Matrix4<f32>,
    pub(crate) scale: f32,
    additional_texture:Option<u32>,
}


impl ModelInstance {
    pub fn new(gl: &gl::Gl,model:Model, scale: f32,additional_texture:Option<&str>) -> ModelInstance {
        let mut additional = None;
        if additional_texture.is_some() {
            let texture=create_texture(gl, &*additional_texture.unwrap());
            additional=Some(texture);
        }
        ModelInstance {
            model,
            matrix: Matrix4::from_translation(vec3(0.0,0.0,0.0)),
            //position,
            scale,
            additional_texture:additional,
        }
    }
    pub fn render(&self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>,our_shader:u32,use_additional:bool) {

        //self.matrix = self.matrix * Matrix4::<f32>::from_angle_y(Deg(1.0));
        let matrix = self.matrix * Matrix4::from_scale(self.scale);

        for sub_model in &self.model.sub_models {
            unsafe {
                //gl.UseProgram(our_shader);
                gl.ActiveTexture(gl::TEXTURE0);
                if use_additional {
                    gl.BindTexture(gl::TEXTURE_2D, self.additional_texture.unwrap());
                } else {
                    gl.BindTexture(gl::TEXTURE_2D, sub_model.texture);
                }
                gl.BindVertexArray(sub_model.vao);

                gl_matrix4(gl, our_shader, matrix, "model");
                gl_matrix4(gl, our_shader, *view, "view");
                gl_matrix4(gl, our_shader, *projection, "projection");

                gl.DrawElements(gl::TRIANGLES, sub_model.indices_len as i32, gl::UNSIGNED_INT, ptr::null());
                gl.BindVertexArray(0);
            }
        }
    }
}