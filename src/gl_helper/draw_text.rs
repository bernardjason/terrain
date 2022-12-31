#![allow(dead_code)]
use crate::{gl, WIDTH, HEIGHT};
//use std::ffi::CString;
//use std::ffi::CStr;
use std::mem;
use std::os::raw::c_void;
use std::path::Path;
use std::ptr;

use cgmath::{Matrix4, vec3, Vector3};
use image::GenericImage;
use crate::gl_helper::shader::create_shader;
use crate::gl_helper::{gl_matrix4, gl_vec3};


const CHARS_PER_LINE: f32 = 31.0;
const CHAR_LINES: f32 = 3.0;

pub struct DrawText {
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
	vec4 t = texture(texture0, TexCoord) ;
	if (t.x > 0.0 ) {
	    t = vec4(use_colour.x,use_colour.y,use_colour.z,1.0);
    }
	FragColor = t;
}";
const VS:&str="#version 300 es
precision mediump float;
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out vec3 use_colour;

uniform mat4 model;
uniform vec3 colour;

void main()
{
	gl_Position = model *  vec4(aPos, 1.0f);
	gl_Position = gl_Position * vec4(1,1,1,1);
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
	use_colour = colour;
}
";

impl DrawText {
    pub fn new(gl: &gl::Gl) -> DrawText {
        let (our_shader, vao, texture1) = unsafe {
            gl.Enable(gl::BLEND);
            gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            let our_shader = create_shader(&gl, VS, FS, None);

            let posx: f32 = 1.0 / CHARS_PER_LINE*0.5;
            let posy: f32 = 1.0 / CHAR_LINES * 0.25;
            let cw: f32 = 1.0 / CHARS_PER_LINE;
            let ch: f32 = 1.0 / CHAR_LINES;
            let mut vertices: [f32; CHARS_PER_LINE as usize * 30 * CHAR_LINES as usize] = [0.0; CHARS_PER_LINE as usize * 30 * CHAR_LINES as usize];
            for y in 0..CHAR_LINES as usize {
                //let offset  = (CHARS_PER_LINE * 30.0) as usize * y  ; //(CHAR_LINES-1.0 - y as f32 ) as usize;
                let offset = (CHARS_PER_LINE * 30.0) as usize * (2 - y); //(CHAR_LINES-1.0 - y as f32 ) as usize;
                for x in 0..CHARS_PER_LINE as usize {
                    let ix: f32 = 0.0;
                    let iy: f32 = 0.0;
                    let imagex: f32 = x as f32 * cw;
                    let imagey: f32 = y as f32 * ch;
                    let v = [
                        ix, iy, 0.0, imagex, imagey,
                        ix + posx, iy, 0.0, imagex + cw, imagey,
                        ix + posx, iy + posy, 0.0, imagex + cw, imagey + ch,
                        ix + posx, iy + posy, 0.0, imagex + cw, imagey + ch,
                        ix, iy + posy, 0.0, imagex, imagey + ch,
                        ix, iy, 0.0, imagex, imagey,
                    ];
                    for i in 0..30 {
                        vertices[offset + i + x * 30] = v[i];
                    }
                }
            }

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
            let img = image::open(&Path::new("resources/font.png")).expect("Failed to load texture");
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

            // XXXXXXXXXXXXXXXXXXXXXXXX our_shader.useProgram();

            (our_shader, vao, texture1)
        };
        DrawText{
            shader: our_shader,
            vao: vao,
            texture: texture1
        }
    }
    pub fn draw_text(&self,gl: &gl::Gl,message: &str, x: f32, y: f32, colour:Vector3<f32>,scale:f32) {
        unsafe {
        gl.ActiveTexture(gl::TEXTURE0);
        gl.BindTexture(gl::TEXTURE_2D, self.texture);
        gl.UseProgram(self.shader);

        gl_vec3(gl, self.shader, colour, "colour"); //        shader.setVec3("lightPos", lightPos);

        gl.BindVertexArray(self.vao);
        }

        let char_vec: Vec<char> = message.chars().collect();
        let scale_x = WIDTH as f32;
        let scale_y = HEIGHT as f32 / 2.0;
        let line_height = 0.1;

        let xx: f32 = x * 2.0;
        let mut yy: f32 = y / scale_y - 1.0;
        let mut letter = 0;
        for c in char_vec {
            if c as u8 > 32 {
                let another_position: [Vector3<f32>; 1] = [vec3(((xx + letter as f32 * 32.0) as f32 / scale_x) - 1.0, yy, 0.0)];
                let model: Matrix4<f32> = Matrix4::from_translation(another_position[0]) * Matrix4::from_scale(scale);
                gl_matrix4(gl, self.shader, model, "model"); //        shader.setMat4("lightSpaceMatrix", lightSpaceMatrix);

                let triangles = if c >= 'A' && c <= '_' {
                    let abcdefg = c as u8 - 'A' as u8;
                    31 * 6 + abcdefg as i32 * 6
                } else if c > '_' {
                    let abcdefg = c as u8 - '`' as u8;
                    31 * 6 * 2 + abcdefg as i32 * 6
                } else if c == '-' {
                    let abcdefg = c as u8 - '"' as u8;
                    abcdefg as i32 * 6
                } else {
                    let abcdefg = c as u8 - '!' as u8;
                    abcdefg as i32 * 6
                };

                unsafe {
                    gl.DrawArrays(gl::TRIANGLES, triangles, 6);
                }
            }
            letter = letter + 1;
            if c as u8 == 10 {
                letter = 0;
                yy =  yy - line_height;
            }
        }

    }
}
