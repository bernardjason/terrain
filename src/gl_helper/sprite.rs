use std::ptr;
use std::str;

use cgmath::{vec3, perspective, Deg, Matrix4 };

use crate::{gl, HEIGHT, WIDTH, SCALE_TO_SCREEN};
use crate::gl_helper::shader::create_shader;
use crate::gl_helper::texture::{create_texture_png, create_texture_jpg};
use crate::gl_helper::vertex::create_vertex;
use crate::gl_helper::gl_matrix4;

const IMAGE_VERTEX_SHADER_SOURCE: &str ="#version 300 es
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aColor;
layout (location = 2) in vec2 aTexCoord;

out vec3 ourColor;
out vec2 TexCoord;

uniform mat4 transform;
uniform mat4 projection;
uniform mat4 view;

void main()
{

	gl_Position = projection * view * transform * vec4(aPos ,1.0) ;
	ourColor = aColor;
	TexCoord = vec2(aTexCoord.x, aTexCoord.y);
}
";

const IMAGE_FRAGMENT_SHADER_SOURCE: &str = "#version 300 es
precision mediump float;
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;

uniform sampler2D texture1;

void main()
{
	FragColor = texture(texture1, TexCoord);
    if(FragColor.a < 0.1)
        discard;
}
";

pub struct Sprite {
    shader_program: u32,
    vao: u32,
    pub texture: u32,
    pub transform: Matrix4<f32>,
    pub rotate: Matrix4<f32>,
    pub forward: Matrix4<f32>,
}



impl Sprite {
    pub fn new(gl: &gl::Gl, x: f32, y: f32, image_file: &str, width: f32, height: f32, texture_ref: Option<u32>) -> Sprite {
        let shader_program = create_shader(&gl, IMAGE_VERTEX_SHADER_SOURCE, IMAGE_FRAGMENT_SHADER_SOURCE);

        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64
        let width = width * 0.5;
        let height = height * 0.5;
        let vertices: [f32; 32] = [
            // positions       // colors        // texture coords
            width, height, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, // top right
            width, -height, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, // bottom right
            -width, -height, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // bottom left
            -width, height, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0  // top left
        ];
        let indices = [
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];

        let vao = create_vertex(&gl, &vertices, &indices);


        let texture = if texture_ref.is_none() {
            if image_file.ends_with(".png") {
                create_texture_png(&gl, image_file)
            } else {
                create_texture_jpg(&gl, image_file)
            }
        } else {
            texture_ref.unwrap()
        };

        Sprite {
            shader_program: shader_program,
            vao: vao,
            texture: texture,
            transform: Matrix4::<f32>::from_translation(vec3(x * SCALE_TO_SCREEN, y * SCALE_TO_SCREEN, 0.0)),
            rotate: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
            forward: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
        }
    }
    pub fn render(&mut self, gl: &gl::Gl) {
        let view: Matrix4<f32> = Matrix4::from_translation(vec3(0., 0., -30.0));
        let projection: Matrix4<f32> = perspective(Deg(45.0), WIDTH as f32 / HEIGHT as f32, 0.1, 100.0);

        unsafe {
            gl.BindTexture(gl::TEXTURE_2D, self.texture);
            gl.UseProgram(self.shader_program);
            gl.BindVertexArray(self.vao);


            //self.transform = self.transform * self.rotate * self.forward;
            gl_matrix4(gl,self.shader_program, self.transform, "model");
            gl_matrix4(gl,self.shader_program, view, "view");
            gl_matrix4(gl,self.shader_program, projection, "projection");

            gl.DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
