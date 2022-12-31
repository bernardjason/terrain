use std::{mem, ptr};
use std::os::raw::c_void;
use std::process::exit;

use cgmath::{Matrix4, Vector3, };

use crate::gl;
use crate::gl_helper::gl_matrix4;
use crate::ground_map::TOTAL_VERTICES_ONE_SIDE;
use crate::pickup::Pickup;
use crate::generate_landscape::GenerateLandscape;

pub struct GroundCell {
    texture: u32,
    vao: u32,
    pub vertice_count: usize,
    position: Vector3<f32>,

    pub vbo: u32,
}

impl GroundCell {
    pub(crate) fn render(&self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>, our_shader: u32) {
        let here = Matrix4::<f32>::from_translation(self.position);
        unsafe {
            //gl.UseProgram(our_shader);
            gl.ActiveTexture(gl::TEXTURE0);
            gl.BindTexture(gl::TEXTURE_2D, self.texture);
            gl.BindVertexArray(self.vao);

            gl_matrix4(gl, our_shader, here, "model");
            gl_matrix4(gl, our_shader, *view, "view");
            gl_matrix4(gl, our_shader, *projection, "projection");
            gl.DrawArrays(gl::TRIANGLES, 0, self.vertice_count as i32);
        }
    }
}

const GRASS_MAX: f32 = 6.0;
pub const SCALE: f32 = 2.0;
pub const SEA: f32 = -0.1;
pub const SAND: f32 = 0.0;
pub const PAST_SAND: f32 = 0.1;

impl GroundCell {
    pub fn new(gl: &gl::Gl, texture: u32, centre_x: f32, centre_z: f32, colour_x: i32, colour_z: i32, landscape: &mut GenerateLandscape,
               pickups:&mut Pickup) -> GroundCell {
        let centre_y = 0.0;

        let (vao, vbo, vertices_count) = {
            let mut vertices: Vec<f32> = vec![];
            let total_vertices_one_side = TOTAL_VERTICES_ONE_SIDE;
            let half_max_vertices_size = total_vertices_one_side / 2;
            //let SCALE = 0.25;

            let image_max = 8.0;

            let mut image_x = colour_x as f32;
            let mut image_z = colour_z as f32;//( colour_z % image_max as i32 )  as f32 ;
            for xx in -half_max_vertices_size..half_max_vertices_size {
                for zz in -half_max_vertices_size..half_max_vertices_size {
                    let x = xx as f32 * SCALE;
                    let z = zz as f32 * SCALE;
                    let add_to = 1.0 * SCALE;
                    let y1 = landscape.pseudo_random((x + centre_x) as f64, (z + centre_z) as f64) as f32;
                    let y2 = landscape.pseudo_random((x + centre_x) as f64, (z + centre_z + add_to) as f64) as f32;
                    let y3 = landscape.pseudo_random((x + centre_x + add_to) as f64, (z + centre_z + add_to) as f64) as f32;
                    let y4 = landscape.pseudo_random((x + centre_x + add_to) as f64, (z + centre_z) as f64) as f32;

                    let colour_x: f32 = (image_x % image_max) as f32 / image_max as f32;
                    let colour_x1: f32 = colour_x + 1.0 / image_max;


                    let colour_z: f32 = if y1 <= SEA && y2 <= SEA && y3 <= SEA && y4 <= SEA {
                        0.0
                    } else {
                        //SOMETHING HERE WAS REALLY BAD
                        //let all = vec![(y1 * 1000.0) as i32, (y2 * 1000.0) as i32, (y3 * 1000.0) as i32, (y4 * 1000.0) as i32, ];
                        //let max = *all.iter().max().unwrap() as f32 + 100.0;
                        //let min = *all.iter().max().unwrap() as f32 + 100.0;
                        let min = 100.0 + 1000.0 * if y1 < y2 && y1 < y3 && y1 < y4 {
                            y1
                        } else if y2 < y1 && y2 < y3 && y2 < y4 {
                            y2
                        } else if y3 < y1 && y3 < y2 && y3 < y4 {
                            y3
                        } else {
                            y4
                        };
                        let max = 100.0 + 1000.0 * if y1 > y2 && y1 > y3 && y1 > y4 {
                            y1
                        } else if y2 > y1 && y2 > y3 && y2 > y4 {
                            y2
                        } else if y3 > y1 && y3 > y2 && y3 > y4 {
                            y3
                        } else {
                            y4
                        };


                        let avg = if min < 75.0 {
                            // let it be SAND
                            0.0
                        } else {
                            if max.round() == 165.0 || max.round() == 167.0 { pickups.new_instance( centre_x+xx as f32,y1,centre_z+zz as f32); }
                            let mut band = max / 500.0 + 1.0;
                            if band >= GRASS_MAX {
                                band = GRASS_MAX
                            }
                            1.0 / image_max * band.round()
                        };
                        avg + 1.0 / image_max
                    };
                    if !(y1 <= SEA && y2 <= SEA && y3 <= SEA && y4 <= SEA) {
                        if colour_z % image_max < 0.125 {
                            exit(0)
                        }
                    }


                    let colour_z1: f32 = colour_z + 1.0 / image_max - 0.01; // take 0.01 off as had rounding flicker on browser

                    let mut cell: Vec<f32> = vec![
                        x, y1, z, colour_x, colour_z,
                        x, y2, (z + add_to), colour_x, colour_z1,
                        (x + add_to), y3, (z + add_to), colour_x1, colour_z1,
                        x, y1, z, colour_x, colour_z,
                        (x + add_to), y3, (z + add_to), colour_x1, colour_z1,
                        (x + add_to), y4, z, colour_x1, colour_z,
                    ];
                    vertices.append(&mut cell);
                    image_x = image_x + 1.0;
                }
                image_x = image_x + image_z;
                image_z = image_z + 1.0;
            }

            let (mut vbo, mut vao) = (0, 0);
            if vertices.len() > 0 {
                unsafe {
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
                }
            }
            (vao, vbo, (vertices.len() as f32 * 0.2) as usize)
        };

        //println!("max {} min {} ",landscape.max,landscape.min);


        GroundCell {
            texture: texture,
            vao: vao,
            vbo: vbo,
            vertice_count: vertices_count,
            position: Vector3::new(centre_x, centre_y, centre_z),
        }
    }
    pub(crate) fn free(&self, gl: &gl::Gl) {
        unsafe {
            gl.DeleteVertexArrays(1, &self.vao);
            gl.DeleteBuffers(1, &self.vbo);
        }
    }
}