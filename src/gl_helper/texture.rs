#![allow(dead_code)]
use std::os::raw::c_void;
use std::path::Path;
use std::str;

use crate::gl;
use image::GenericImage;

pub fn create_texture(gl: &gl::Gl, filename: &str) -> u32 {
    if filename.ends_with("png") {
        create_texture_png(gl,filename)
    } else {
        create_texture_jpg(gl,filename)
    }
}
pub fn create_texture_png(gl: &gl::Gl, filename: &str) -> u32 {
    unsafe {
        let mut texture: u32 = 0;
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // load image, create texture and generate mipmaps

        let img = image::open(&Path::new(filename)).expect("Failed to load texture");
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
        texture
    }
}
pub fn create_texture_jpg(gl: &gl::Gl, filename: &str) -> u32 {
    unsafe {
        let mut texture: u32 = 0;
        gl.GenTextures(1, &mut texture);
        gl.BindTexture(gl::TEXTURE_2D, texture); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

        // load image, create texture and generate mipmaps

        let img = image::open(&Path::new(filename)).expect("Failed to load texture");
        let data = img.flipv().raw_pixels();
        gl.TexImage2D(gl::TEXTURE_2D,
                      0,
                      gl::RGB as i32,
                      img.width() as i32,
                      img.height() as i32,
                      0,
                      gl::RGB,
                      gl::UNSIGNED_BYTE,
                      &data[0] as *const u8 as *const c_void);
        gl.GenerateMipmap(gl::TEXTURE_2D);
        texture
    }
}
