extern crate cgmath;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::*;

//use crate::gl_helper::shader::create_shader;
use crate::gl_helper::texture::{create_texture_jpg, create_texture_png};
use crate::gl_helper::gl_matrix4;
use crate::{gl};





pub struct CubeInstance {
    pub id: u128,
    pub matrix: Matrix4<f32>,
}

impl PartialEq for CubeInstance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct Cube {
    //our_shader: u32,
    pub texture: u32,
    vao: u32,
}

impl Cube {
    pub fn new(gl: &gl::Gl, image_file: &str, size: Vector3<f32>, texture_end: f32) -> Cube {
        let ( _vbo, vao, texture) = unsafe {
            //let our_shader = create_shader(&gl, IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE);

            let vertices: [f32; 180] = [
// positions       // texture coords
                -size.x, -size.y, -size.z, 0.0, 0.0,
                size.x, -size.y, -size.z, texture_end, 0.0,
                size.x, size.y, -size.z, texture_end, texture_end,
                size.x, size.y, -size.z, texture_end, texture_end,
                -size.x, size.y, -size.z, 0.0, texture_end,
                -size.x, -size.y, -size.z, 0.0, 0.0,
                -size.x, -size.y, size.z, 0.0, 0.0,
                size.x, -size.y, size.z, texture_end, 0.0,
                size.x, size.y, size.z, texture_end, texture_end,
                size.x, size.y, size.z, texture_end, texture_end,
                -size.x, size.y, size.z, 0.0, texture_end,
                -size.x, -size.y, size.z, 0.0, 0.0,
                -size.x, size.y, size.z, texture_end, 0.0,
                -size.x, size.y, -size.z, texture_end, texture_end,
                -size.x, -size.y, -size.z, 0.0, texture_end,
                -size.x, -size.y, -size.z, 0.0, texture_end,
                -size.x, -size.y, size.z, 0.0, 0.0,
                -size.x, size.y, size.z, texture_end, 0.0,
                size.x, size.y, size.z, texture_end, 0.0,
                size.x, size.y, -size.z, texture_end, texture_end,
                size.x, -size.y, -size.z, 0.0, texture_end,
                size.x, -size.y, -size.z, 0.0, texture_end,
                size.x, -size.y, size.z, 0.0, 0.0,
                size.x, size.y, size.z, texture_end, 0.0,
                -size.x, -size.y, -size.z, 0.0, texture_end,
                size.x, -size.y, -size.z, texture_end, texture_end,
                size.x, -size.y, size.z, texture_end, 0.0,
                size.x, -size.y, size.z, texture_end, 0.0,
                -size.x, -size.y, size.z, 0.0, 0.0,
                -size.x, -size.y, -size.z, 0.0, texture_end,
                -size.x, size.y, -size.z, 0.0, texture_end,
                size.x, size.y, -size.z, texture_end, texture_end,
                size.x, size.y, size.z, texture_end, 0.0,
                size.x, size.y, size.z, texture_end, 0.0,
                -size.x, size.y, size.z, 0.0, 0.0,
                -size.x, size.y, -size.z, 0.0, texture_end,
            ];
            let (mut vbo, mut vao) = (0, 0);
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);

            gl.BindVertexArray(vao);

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(gl::ARRAY_BUFFER,
                          (vertices.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                          &vertices[0] as *const f32 as *const c_void,
                          gl::STATIC_DRAW);

            let stride = 5 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
            gl.EnableVertexAttribArray(1);


            let texture = if image_file.ends_with(".png") {
                create_texture_png(&gl, image_file)
            } else if image_file.ends_with(".jpg") {
                create_texture_jpg(&gl, image_file)
            } else {
                0
            };


            ( vbo, vao, texture)
        };

        Cube {
            texture: texture,
            vao,
        }
    }


    pub fn render(&mut self, gl: &gl::Gl, matrix: &Matrix4<f32>, view: &Matrix4<f32>, projection: &Matrix4<f32>,our_shader:u32,texture:u32) {
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, texture);
            gl.BindVertexArray(self.vao);
            gl_matrix4(gl, our_shader, *matrix, "model");
            gl_matrix4(gl, our_shader, *view, "view");
            gl_matrix4(gl, our_shader, *projection, "projection");
            gl.DrawArrays(gl::TRIANGLES, 0, 36);
        }
    }
}

