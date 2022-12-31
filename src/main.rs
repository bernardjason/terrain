use std::time::Instant;

use cgmath::{Matrix4, Vector4};

use crate::runtime::Runtime;

mod gl;
mod gl_helper;
mod runtime;
mod flying_camera;
mod ground_cell;
mod skybox_shader;
mod handle_javascript;
mod ground_map;
mod landscape;
mod generate_landscape;
mod plane;
mod alienship;
mod player;
mod bullet;
mod target_xz;
mod target_y;
mod target_roll_xz;
mod target_even_keel;
mod openglshadow;
mod shadow_shaders;
mod pickup;
mod special_effects;
mod cube;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const FAR: f32 = 200.0;

fn main() {
    let runtime = Runtime::new();

    emscripten_main_loop::run(runtime);
}

pub fn get_start_time() -> Instant {
    let start = Instant::now();
    return start;
}

pub fn output_elapsed(start: Instant, msg: &str) {
    let duration = start.elapsed();
    println!("*********** {} {:?}", msg, duration);
}
pub fn print_matrix(m: Matrix4<f32>) {
    println!("x= {}", get_vector4_as_string(m.x));
    println!("y= {}", get_vector4_as_string(m.y));
    println!("z= {}", get_vector4_as_string(m.z));
    println!("w= {}", get_vector4_as_string(m.w));
}

pub fn get_vector4_as_string(v: Vector4<f32>) -> String {
    let s = format!("{},{},{},{}", v.x, v.y, v.z, v.w);
    return s;
}
