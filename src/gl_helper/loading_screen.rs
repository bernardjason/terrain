#![allow(dead_code)]

use crate::{gl,  };
//use std::ffi::CString;
//use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

//use cgmath::{Matrix4, vec3, Vector3};
use image::GenericImage;
use crate::gl_helper::shader::create_shader;


pub struct LoadingScreen {
    shader: u32,
    vao: u32,
    texture: u32,
}

const FS:&str = "#version 300 es
precision mediump float;
out vec4 FragColor;
in vec2 TexCoord;
in vec3 use_colour;

uniform sampler2D texture0;

void main()
{
	FragColor = texture(texture0, TexCoord);
}";
const VS:&str="#version 300 es
precision mediump float;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;

uniform mat4 model;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}
";

impl LoadingScreen {
    pub fn new(gl: &gl::Gl,image_name:&str) -> LoadingScreen {
        let (our_shader, vao, texture1) = unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let our_shader = create_shader(&gl, VS, FS, None);


            let size = 0.5;
            let vertices: [f32; 30] = [
                -size, -size,0.0,    0.0,0.0,
                -size,  size,0.0,    0.0,1.0,
                size,   size,0.0,    1.0,1.0,
                -size, -size,0.0,    0.0,0.0,
                size,   size,0.0,    1.0,1.0,
                size,  -size,0.0,    1.0,0.0
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


            let mut texture1 = 0;
            gl.GenTextures(1, &mut texture1);
            gl.BindTexture(gl::TEXTURE_2D, texture1);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            let img = image::open(&Path::new(image_name)).expect("Failed to load texture");
            let data = img.flipv().raw_pixels();
            gl.TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGBA as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           gl::RGBA,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void);
            gl.GenerateMipmap(gl::TEXTURE_2D);


            (our_shader, vao, texture1)
        };
        LoadingScreen{
            shader: our_shader,
            vao,
            texture: texture1
        }
    }
    pub fn render(&self,gl: &gl::Gl) {
        unsafe {
        gl.ActiveTexture(gl::TEXTURE0);
        gl.BindTexture(gl::TEXTURE_2D, self.texture);
        gl.UseProgram(self.shader);

        gl.BindVertexArray(self.vao);
        gl.DrawArrays(gl::TRIANGLES, 0, 6);
        }

    }
}
