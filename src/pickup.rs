extern crate cgmath;

use std::collections::HashMap;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::*;

use crate::gl;
use crate::gl_helper::{gl_matrix4, gl_vec3};

use self::gl::types::*;
use crate::gl_helper::texture::create_texture_png;

pub struct PickupInstance {
    pub position: Vector3<f32>,
    colour: Vector3<f32>,
    pub clicks: i32,
    picked_up: bool,
}

impl PickupInstance {
    pub fn decrease(&mut self) -> i32 {
        self.clicks=self.clicks-1;
        self.clicks
    }
}

pub struct Pickup {
    vao: u32,
    texture:u32,
    pub onmap: HashMap<i32, PickupInstance>,
    //pub instances: Vec<PickupInstance>,
}

const PICKUP_TIME: i32 = 1200;

impl Pickup {
    pub fn new(gl: &gl::Gl) -> Pickup {
        let ( vao,texture) = unsafe {

            let texture = create_texture_png(&gl, "resources/prize.png");

            let size = 0.2;
            let hsize = 0.40;
            let vertices: [f32; 180] = [
                // positions       // texture coords   8 blocks
                -size, -hsize, -size, 0.0, 0.0,
                size, -hsize, -size, 0.125, 0.0,
                size, hsize, -size, 0.125, 1.0,
                size, hsize, -size, 0.125, 1.0,
                -size, hsize, -size, 0.0, 1.0,
                -size, -hsize, -size, 0.0, 0.0,

                -size, -hsize, size, 0.125, 0.0,
                size, -hsize, size, 0.250, 0.0,
                size, hsize, size, 0.250, 1.0,
                size, hsize, size, 0.250, 1.0,
                -size, hsize, size, 0.125, 1.0,
                -size, -hsize, size, 0.125, 0.0,


                -size, hsize, size, 0.375, 0.0,
                -size, hsize, -size, 0.375, 1.0,
                -size, -hsize, -size, 0.250, 1.0,
                -size, -hsize, -size, 0.250, 1.0,
                -size, -hsize, size, 0.250, 0.0,
                -size, hsize, size, 0.375, 0.0,

                size, hsize, size, 0.37500, 0.0,
                size, hsize, -size, 0.37500, 1.0,
                size, -hsize, -size, 0.5, 1.0,
                size, -hsize, -size, 0.5, 1.0,
                size, -hsize, size, 0.5, 0.0,
                size, hsize, size, 0.37500, 0.0,

                -size, -hsize, -size, 0.500, 1.0,
                size, -hsize, -size, 0.625, 1.0,
                size, -hsize, size, 0.625, 0.0,
                size, -hsize, size, 0.625, 0.0,
                -size, -hsize, size, 0.500, 0.0,
                -size, -hsize, -size, 0.500, 1.0,

                -size, hsize, -size, 0.625, 1.0, // fine
                size, hsize, -size, 0.750, 1.0,
                size, hsize, size, 0.750, 0.0,
                size, hsize, size, 0.750, 0.0,
                -size, hsize, size, 0.625, 0.0,
                -size, hsize, -size, 0.625, 1.0
            ];
            let (mut vbo, mut vao) = (0, 0);
            gl.GenVertexArrays(1, &mut vao);
            gl.GenBuffers(1, &mut vbo);

            gl.BindVertexArray(vao);

            gl.BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl.BufferData(gl::ARRAY_BUFFER,
                          (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                          &vertices[0] as *const f32 as *const c_void,
                          gl::STATIC_DRAW);

            let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;
            gl.VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl.EnableVertexAttribArray(0);
            gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (3 * mem::size_of::<GLfloat>()) as *const c_void);
            gl.EnableVertexAttribArray(1);


            (vao,texture)
        };

        Pickup {
            vao,
            texture,
            onmap: HashMap::new(),
            //instances: Vec::<PickupInstance>::new(),
        }
    }

    pub fn new_instance(&mut self, x: f32, y: f32, z: f32) {
        let key = x as i32 * 1024 + z as i32;
        if !self.onmap.contains_key(&key) {
            let instance = PickupInstance {
                position: vec3(x, y*6.0, z),
                clicks: PICKUP_TIME,
                colour: vec3(1.0, 0.0, 1.0),
                picked_up: false,
            };
            self.onmap.insert(key, instance);
            //println!("Added as {},{},{}", x, y, z);
        }
    }

    /*
    try and age old prizes as well as remove picked up ones. Assuming that when you fly around the old
    ones can disappear in background
     */
    pub fn update_pickups(&mut self, _delta_time: f32,player_position:&Vector3<f32>) -> i32 {
        let mut remove_keys:Vec<i32> = vec![];
        let mut picked_up = 0;
        for (k,i) in self.onmap.iter_mut() {
            if ! i.picked_up {
                if player_position.distance2(i.position) < 0.3 {
                  i.picked_up = true;
                    picked_up=picked_up+1;
                }
            }

            if i.decrease() < 0 {
                remove_keys.push(*k);
            }
        }
        for k in remove_keys {
            self.onmap.remove_entry(&k);
        }
        picked_up
    }
    pub fn render(&mut self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>,our_shader:u32) {
        unsafe {
            //gl.UseProgram(self.our_shader);
            gl.UseProgram(our_shader);
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.texture);

            gl_matrix4(gl, our_shader, *view, "view");
            gl_matrix4(gl, our_shader, *projection, "projection");


            gl.BindVertexArray(self.vao);
        }

        for i in self.onmap.values() {
            //let b = self.onmap.get(i).unwrap();
            if ! i.picked_up {
                let matrix = Matrix4::<f32>::from_translation(i.position);
                unsafe {
                    let colour = vec3(i.colour.x, i.colour.y, i.colour.z);
                    gl_matrix4(gl, our_shader, matrix, "model");
                    gl_vec3(gl, our_shader, colour, "colour");
                    gl.DrawArrays(gl::TRIANGLES, 0, 36);
                }
            }
        }
    }
    /*
    pub(crate) fn free(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteVertexArrays(1, &self.vao);
            gl.DeleteBuffers(1, &self.vbo);
        }
    }
     */
}


