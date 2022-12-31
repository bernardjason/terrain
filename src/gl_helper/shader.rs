#![allow(dead_code)]
use std::{ptr, str};
use std::ffi::CString;

use crate::gl;

pub fn create_shader(gl: &gl::Gl, image_vertex_shader_source:&str, image_fragment_shader_source:&str, geometry_shader:Option<&str>) -> u32 {
    unsafe {
        let mut log_size:i32=8192;
        let mut success = gl::FALSE as gl::types::GLint;
        let mut info_log = Vec::with_capacity((log_size + 1) as usize);
        info_log.set_len((log_size - 1) as usize); // subtract 1 to skip the trailing null character

        let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(image_vertex_shader_source.as_bytes()).unwrap();
        gl.ShaderSource(vertex_shader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl.CompileShader(vertex_shader);
        gl.GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl.GetShaderInfoLog(vertex_shader, log_size, &mut log_size, info_log.as_mut_ptr() as *mut gl::types::GLchar);
            info_log.set_len(log_size as usize);
            panic!("shader error {}",String::from_utf8_lossy(&info_log));
        }

        let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(image_fragment_shader_source.as_bytes()).unwrap();
        gl.ShaderSource(fragment_shader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl.CompileShader(fragment_shader);
        gl.GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl.GetShaderInfoLog(vertex_shader, log_size, &mut log_size, info_log.as_mut_ptr() as *mut gl::types::GLchar);
            info_log.set_len(log_size as usize);
            panic!("shader error {}",String::from_utf8_lossy(&info_log));
        }

        let shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program, vertex_shader);
        gl.AttachShader(shader_program, fragment_shader);

        geometry_shader.map(|geo| {
            let geo_shader = gl.CreateShader(gl::GEOMETRY_SHADER);
            let c_str_geo = CString::new(geo.as_bytes()).unwrap();
            gl.ShaderSource(geo_shader, 1, &c_str_geo.as_ptr(), ptr::null());
            gl.CompileShader(geo_shader);
            gl.GetShaderiv(geo_shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as gl::types::GLint {
                gl.GetShaderInfoLog(vertex_shader, log_size, &mut log_size, info_log.as_mut_ptr() as *mut gl::types::GLchar);
                info_log.set_len(log_size as usize);
                panic!("shader error {}",String::from_utf8_lossy(&info_log));
            }
            gl.AttachShader(shader_program, geo_shader);
        });

        gl.LinkProgram(shader_program);
        gl.GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as gl::types::GLint {
            gl.GetShaderInfoLog(vertex_shader, log_size, &mut log_size, info_log.as_mut_ptr() as *mut gl::types::GLchar);
            info_log.set_len(log_size as usize);
            panic!("shader error {}",String::from_utf8_lossy(&info_log));
        }
        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);
        shader_program
    }
}