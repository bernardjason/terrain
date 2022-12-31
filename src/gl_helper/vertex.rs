#![allow(dead_code)]
use std::{mem, ptr};
use std::os::raw::c_void;

use crate::gl;

pub fn create_vertex(gl: &gl::Gl, vertices: &[f32], indices: &[i32]) -> u32 {

    // set up vertex data (and buffer(s)) and configure vertex attributes
    // ------------------------------------------------------------------
    // HINT: type annotation is crucial since default for float literals is f64
    /*
    let vertices: [f32; 32] = [
        // positions       // colors        // texture coords
        0.5, 0.5, 0.0,      0.0, 0.0, 0.0, 1.0, 1.0, // top right
        0.5, -0.5, 0.0,     0.0, 0.0, 0.0, 1.0, 0.0, // bottom right
        -0.5, -0.5, 0.0,    0.0, 0.0, 0.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0,     0.0, 0.0, 0.0, 0.0, 1.0  // top left
    ];
    let indices = [
        0, 1, 3,  // first Triangle
        1, 2, 3   // second Triangle
    ];
    */

    let (mut vbo, mut vao, mut ebo) = (0, 0, 0);

    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.GenBuffers(1, &mut vbo);
        gl.GenBuffers(1, &mut ebo);

        gl.BindVertexArray(vao);

        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl.BufferData(gl::ARRAY_BUFFER,
                      (vertices.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                      &vertices[0] as *const f32 as *const c_void,
                      gl::STATIC_DRAW);

        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,
                      (indices.len() * mem::size_of::<gl::types::GLfloat>()) as gl::types::GLsizeiptr,
                      &indices[0] as *const i32 as *const c_void,
                      gl::STATIC_DRAW);

        let stride = 8 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;
        // position attribute
        gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl.EnableVertexAttribArray(0);
        // color attribute
        gl.VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
        gl.EnableVertexAttribArray(1);
        // texture coord attribute
        gl.VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, (6 * mem::size_of::<gl::types::GLfloat>()) as *const c_void);
        gl.EnableVertexAttribArray(2);

        //gl.BindBuffer(gl::ARRAY_BUFFER, 0);

        vao
    }
}