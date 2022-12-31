#![allow(dead_code)]

use std::ffi::CString;

use cgmath::{Array, Matrix, Matrix4, Vector2, Vector3};

use crate::gl;

pub(crate) mod texture;
pub(crate) mod shader;
pub(crate) mod model;
pub mod instance_model;
pub mod draw_text;
pub(crate) mod loading_screen;
mod vertex;
pub mod skybox;

pub fn gl_matrix4(gl: &gl::Gl, shader_program: u32, mat4: Matrix4<f32>, name: &str) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
            let location = gl.GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
        gl.UniformMatrix4fv(
            location,
            1,
            gl::FALSE,
            mat4.as_ptr(),
        );
    }
}

pub fn gl_vec3(gl: &gl::Gl, shader_program: u32, vec3: Vector3<f32>, name: &str) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
            let location = gl.GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
        gl.Uniform3fv(location, 1, vec3.as_ptr());
    }
}

pub fn gl_vec2(gl: &gl::Gl, shader_program: u32, vec2: Vector2<f32>, name: &str) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
            let location = gl.GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
        gl.Uniform2fv(location, 1, vec2.as_ptr());
    }
}

pub fn gl_int(gl: &gl::Gl, shader_program: u32, value: i32, name: &str) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
            let location = gl.GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
        gl.Uniform1i(location, value);
    }
}

pub fn gl_float(gl: &gl::Gl, shader_program: u32, value: f32, name: &str) {
    unsafe {
        #[allow(temporary_cstring_as_ptr)]
            let location = gl.GetUniformLocation(shader_program, CString::new(name).unwrap().as_ptr());
        gl.Uniform1f(location, value);
    }
}


