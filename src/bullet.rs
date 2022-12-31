extern crate cgmath;
use crate::gl;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

use cgmath::*;
use rand::Rng;


use self::gl::types::*;
use crate::gl_helper::shader::create_shader;
use crate::gl_helper::{gl_matrix4, gl_vec3};

pub struct BulletInstance {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    colour:Vector3<f32>,
    pub clicks:i32,
    pub enemy:bool,
}
impl BulletInstance {
    pub fn mark_finished(&mut self) {
        self.clicks = 99999999;
    }
}

pub struct Bullet {
    our_shader: u32,
    vao: u32,
    colour:Vector3<f32>,
    pub instances: Vec<BulletInstance>,
}

impl Bullet {
    pub fn new(gl: &gl::Gl) -> Bullet {
        let  our_shader = create_shader(&gl, BULLET_VS, BULLET_FS, None);
        let (vbo, vao) = unsafe {

            let size = 0.03;
            let vertices: [f32; 180] = [
                // positions       // texture coords
                -size, -size, -size, 0.0, 0.0,
                size, -size, -size, 1.0, 0.0,
                size, size, -size, 1.0, 1.0,
                size, size, -size, 1.0, 1.0,
                -size, size, -size, 0.0, 1.0,
                -size, -size, -size, 0.0, 0.0,
                -size, -size, size, 0.0, 0.0,
                size, -size, size, 1.0, 0.0,
                size, size, size, 1.0, 1.0,
                size, size, size, 1.0, 1.0,
                -size, size, size, 0.0, 1.0,
                -size, -size, size, 0.0, 0.0,
                -size, size, size, 1.0, 0.0,
                -size, size, -size, 1.0, 1.0,
                -size, -size, -size, 0.0, 1.0,
                -size, -size, -size, 0.0, 1.0,
                -size, -size, size, 0.0, 0.0,
                -size, size, size, 1.0, 0.0,
                size, size, size, 1.0, 0.0,
                size, size, -size, 1.0, 1.0,
                size, -size, -size, 0.0, 1.0,
                size, -size, -size, 0.0, 1.0,
                size, -size, size, 0.0, 0.0,
                size, size, size, 1.0, 0.0,
                -size, -size, -size, 0.0, 1.0,
                size, -size, -size, 1.0, 1.0,
                size, -size, size, 1.0, 0.0,
                size, -size, size, 1.0, 0.0,
                -size, -size, size, 0.0, 0.0,
                -size, -size, -size, 0.0, 1.0,
                -size, size, -size, 0.0, 1.0,
                size, size, -size, 1.0, 1.0,
                size, size, size, 1.0, 0.0,
                size, size, size, 1.0, 0.0,
                -size, size, size, 0.0, 0.0,
                -size, size, -size, 0.0, 1.0
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



            (vbo, vao)
        };

        Bullet {
            our_shader,
            vao,
            colour:vec3(1.0,1.0,0.0),
            instances: Vec::<BulletInstance>::new(),
        }
    }

    pub fn new_instance(&mut self, direction: Vector3<f32>, position: Vector3<f32>,colour:Vector3<f32>,enemy:bool) {
        let instance = BulletInstance {
            position,
            direction,
            clicks:0,
            colour,
            enemy,
        };
        self.instances.push(instance);

    }

    pub fn update_bullets(&mut self, delta_time:f32) {
        let speed = 8.0f32;

        let mut i = self.instances.len();
        while i >= 1 {
            i=i-1;
            let mut b = self.instances.get_mut(i).unwrap();
            b.position -= b.direction * delta_time * speed;
            b.clicks = b.clicks +1;
            if b.clicks > 80 {
                self.instances.remove(i);
            }
        }
    }
    pub fn render(&mut self, gl: &gl::Gl,view:&Matrix4<f32>, projection:&Matrix4<f32>) {
        let mut rng = rand::thread_rng();
        unsafe {
            gl.UseProgram(self.our_shader);
            gl_matrix4(gl, self.our_shader, *view, "view");
            gl_matrix4(gl, self.our_shader, *projection, "projection");
            gl.BindVertexArray(self.vao);
        }
        self.colour.x = self.colour.x + rng.gen_range(0.1,0.4);
        self.colour.y = self.colour.y + rng.gen_range(0.0,0.4);
        if self.colour.x > 1.0 { self.colour.x = 0.5; }
        if self.colour.y > 1.0 { self.colour.y = 0.5; }

        for i in 0..self.instances.len() {
            let b = self.instances.get(i).unwrap();
            let matrix = Matrix4::<f32>::from_translation(b.position );
            unsafe {

                let colour = vec3(b.colour.x + self.colour.x+0.4,b.colour.y + self.colour.y,b.colour.z + self.colour.z);
                gl_matrix4(gl, self.our_shader, matrix, "model");
                gl_vec3(gl, self.our_shader, colour, "colour");
                gl.DrawArrays(gl::TRIANGLES, 0, 36);
            }

        }

    }


}

pub const BULLET_FS:&str = "#version 300 es
precision lowp float;
out vec4 FragColor;

in vec2 TexCoord;
in vec3 use_colour;

// texture samplers
uniform sampler2D texture0;

void main()
{
        vec4 t = texture(texture0, TexCoord) ;
        t = vec4(use_colour.x,use_colour.y,use_colour.z,1.0);
        FragColor = t;
}
";
pub const BULLET_VS:&str = "#version 300 es
precision lowp float;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out vec3 use_colour;

uniform mat4 model;
uniform vec3 colour;
uniform mat4 view;
uniform mat4 projection;

void main()
{
        gl_Position = projection * view * model * vec4(aPos, 1.0f);
        //gl_Position = gl_Position * vec4(1,1,1,1);
        TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        use_colour = colour;
}
";


