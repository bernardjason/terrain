#![allow(dead_code)]
extern crate cgmath;

use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::*;

//use crate::gl_helper::shader::create_shader;
use crate::gl_helper::texture::{create_texture_jpg, create_texture_png};
use crate::gl_helper::gl_matrix4;
use crate::{gl};
use crate::gl_helper::shader::create_shader;

pub const SKYBOX_VS:&str = "#version 300 es
precision lowp float;
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out vec3 Pos;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
	gl_Position = projection * view * model * vec4(aPos, 1.0f);
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
	Pos = vec3(aPos);
}
";

pub const SKYBOX_FS:&str = "#version 300 es
precision lowp float;
out vec4 FragColour;

in vec2 TexCoord;
in vec3 Pos;

// texture samplers
uniform sampler2D texture0;

void main()
{
	FragColour = texture(texture0, TexCoord);
    FragColour = FragColour * 0.5;

}
";

pub struct SkyboxInstance {
    pub id: u128,
    pub matrix: Matrix4<f32>,
}

impl PartialEq for SkyboxInstance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct Skybox {
    pub texture: u32,
    shader: u32,
    vao: u32,
}

impl Skybox {
    pub fn new(gl: &gl::Gl, image_file: &str, ) -> Skybox {
        let big=70.0;
        let size = vec3(big,big,big);
        let texture_end = 1.0;
        let ( _vbo, vao, texture,shader) = unsafe {

            let shader = create_shader(&gl, SKYBOX_VS, SKYBOX_FS, None);

            let vertices: [f32; 180] = [
                // positions       // texture coords
                -size.x, -size.y, -size.z,    0.0, 0.0,
                size.x, -size.y, -size.z,    0.16, 0.0,
                size.x, size.y, -size.z,    0.16, texture_end,
                size.x, size.y, -size.z,    0.16, texture_end,
                -size.x, size.y, -size.z,    0.0, texture_end,
                -size.x, -size.y, -size.z,    0.0, 0.0,


                -size.x, -size.y, size.z,    0.0, 0.0,
                size.x, -size.y, size.z,    0.16, 0.0,
                size.x, size.y, size.z,    0.16, texture_end,
                size.x, size.y, size.z,    0.16, texture_end,
                -size.x, size.y, size.z,    0.0, texture_end,
                -size.x, -size.y, size.z,    0.0, 0.0,

                // EAST
                -size.x, size.y, size.z,     1.0, 0.0,
                -size.x, size.y, -size.z,    1.0, texture_end,
                -size.x, -size.y, -size.z,   0.84, texture_end,
                -size.x, -size.y, -size.z,   0.84, texture_end,
                -size.x, -size.y, size.z,    0.84, 0.0,
                -size.x, size.y, size.z,     1.0, 0.0,




                // WEST
                size.x, size.y, size.z,    1.0, 0.0,
                size.x, size.y, -size.z,    1.0, texture_end,
                size.x, -size.y, -size.z,    0.84, texture_end,
                size.x, -size.y, -size.z,    0.84, texture_end,
                size.x, -size.y, size.z,    0.84, 0.0,
                size.x, size.y, size.z,    1.0, 0.0,


                -size.x, -size.y, -size.z,    0.64, texture_end,
                size.x, -size.y, -size.z,    0.80, texture_end,
                size.x, -size.y, size.z,    0.80, 0.0,
                size.x, -size.y, size.z,    0.80, 0.0,
                -size.x, -size.y, size.z,    0.64, 0.0,
                -size.x, -size.y, -size.z,    0.64, texture_end,


                -size.x, size.y, -size.z,    0.80, texture_end,
                size.x, size.y, -size.z,    0.96, texture_end,
                size.x, size.y, size.z,    0.96, 0.0,
                size.x, size.y, size.z,    0.96, 0.0,
                -size.x, size.y, size.z,    0.80, 0.0,
                -size.x, size.y, -size.z,    0.80, texture_end,
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
            } else {
                create_texture_jpg(&gl, image_file)
            };


            ( vbo, vao, texture,shader)
        };

        Skybox {
            texture: texture,
            shader,
            vao,
        }
    }


    pub fn render(&mut self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>,position:Vector3<f32>) {
        let matrix = Matrix4::from_translation(position);
        unsafe {
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.texture);
            gl.BindVertexArray(self.vao);
            gl.UseProgram(self.shader);
            gl_matrix4(gl, self.shader, matrix, "model");
            gl_matrix4(gl, self.shader, *view, "view");
            gl_matrix4(gl, self.shader, *projection, "projection");
            gl.DrawArrays(gl::TRIANGLES, 0, 36);
        }

    }
}

