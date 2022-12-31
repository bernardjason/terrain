#![allow(dead_code)]
#![allow(deref_nullptr)]
use std::os::raw::c_void;
use std::ptr;




use cgmath::{vec2, vec3, Vector3, Vector2};

use crate::gl;
use crate::gl_helper::texture::{create_texture };
use std::mem::size_of;
//use crate::gl_helper::shader::create_shader;

#[repr(C)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub tex_coords: Vector2<f32>,
}

/// Get offset to struct member, similar to `offset_of` in C/C++
/// From https://stackoverflow.com/questions/40310483/how-to-get-pointer-offset-in-bytes/40310851#40310851
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        &(*(ptr::null() as *const $ty)).$field as *const _ as usize
    }
}




#[derive(Clone)]
pub struct SubModel {
    pub(crate) texture: u32,
    pub indices_len: usize,
    pub(crate) vao: u32,
}

#[derive(Clone)]
pub struct Model {
    pub(crate) sub_models: Vec<SubModel>,

}

impl Model {
    pub fn new(gl: &gl::Gl, path: &str,image_file:&str) -> Model {
        let cornell_box = tobj::load_obj(path.as_ref());
        assert!(cornell_box.is_ok());
        let (models, _materials) = cornell_box.unwrap();

        let texture = create_texture(&gl, image_file);

        let mut sub_models = Vec::<SubModel>::new();
        for model in models.iter() {
            let mut vertices: Vec<Vertex> = Vec::new();
            let mut indices: Vec<u32> = Vec::new();
            let mesh = &model.mesh;
            indices.append(&mut model.mesh.indices.clone());
            let num_vertices = mesh.positions.len() / 3;
            let (p,  t) = (&mesh.positions, &mesh.texcoords);
            for i in 0..num_vertices {
                vertices.push(Vertex {
                    position: vec3(p[i * 3], p[i * 3 + 1], p[i * 3 + 2]),
                    tex_coords: vec2(t[i * 2], t[i * 2 + 1]),
                })
            }
            vertices.shrink_to_fit();
            indices.shrink_to_fit();
            let indices_len = indices.len();
            let vao = setup_mesh(gl, vertices, indices);
            let sub_model = SubModel {
                texture,
                indices_len,
                vao,
            };
            sub_models.push(sub_model);
        }

        //let our_shader = create_shader(&gl, IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE);

        Model {
            sub_models,
        }
    }

}

fn setup_mesh(gl: &gl::Gl, vertices: Vec<Vertex>, indices: Vec<u32>) -> u32 {
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;

    unsafe {
        gl.GenVertexArrays(1, &mut vao);
        gl.GenBuffers(1, &mut vbo);
        gl.GenBuffers(1, &mut ebo);

        gl.BindVertexArray(vao);
        gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
        let size = (vertices.len() * size_of::<Vertex>()) as isize;
        let data = &vertices[0] as *const Vertex as *const c_void;
        gl.BufferData(gl::ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        let size = (indices.len() * size_of::<u32>()) as isize;
        let data = &indices[0] as *const u32 as *const c_void;
        gl.BufferData(gl::ELEMENT_ARRAY_BUFFER, size, data, gl::STATIC_DRAW);

        let size = size_of::<Vertex>() as i32;
        gl.EnableVertexAttribArray(0);
        gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, position) as *const c_void);

        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, size, offset_of!(Vertex, tex_coords) as *const c_void);

        gl.BindVertexArray(0);
    }
    vao
}

