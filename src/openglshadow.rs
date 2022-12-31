extern crate cgmath;

//use std::fs;
use std::ptr;

use cgmath::*;
use cgmath::{vec3};

use crate::gl_helper::shader::create_shader;
//use crate::gl_helper::texture::{ create_texture_png};
use crate::gl_helper::{gl_matrix4, gl_vec3, gl_int, };
use crate::{gl, WIDTH, HEIGHT, };
use crate::shadow_shaders::*;

pub const SHADOW_WIDTH: i32 = 2048;
pub const SHADOW_HEIGHT: i32 = 2048;

pub struct OpenglShadowInstance {
    pub id: u128,
    pub matrix: Matrix4<f32>,
}

impl PartialEq for OpenglShadowInstance {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

fn vec2point(vector:Vector3<f32>) -> Point3<f32> {
    let p = Point3::new(vector.x,vector.y,vector.z);
    return p
}

pub struct OpenglShadow {
    depth_map_fbo: u32,
    depth_map: u32,
    pub simple_depth_shader: u32,
    pub shader: u32,
    light_pos: Vector3<f32>,
    light_space_matrix:Matrix4<f32>,

}

impl OpenglShadow {
    const NEAR_PLANE:f32 = 1.0f32;
    const FAR_PLANE:f32 = 20.0f32;
    pub fn new(gl: &gl::Gl) -> OpenglShadow {

        let shader = create_shader(&gl, SHADOW_MAPPING_VS, SHADOW_MAPPING_FS, None);
        let simple_depth_shader = create_shader(&gl,
                                                SHADOW_MAPPING_DEPTH_VS, SHADOW_MAPPING_DEPTH_FS, None);


        let (depth_map_fbo, depth_map) = {
            let mut depth_map_fbo: u32 = 0;
            let mut depth_map: u32 = 0;
            unsafe {
                gl.GenFramebuffers(1, &mut depth_map_fbo); //        glGenFramebuffers(1, &depthMapFBO);

                gl.GenTextures(1, &mut depth_map); //      glGenTextures(1, &depthMap);
                gl.BindTexture(gl::TEXTURE_2D, depth_map); //       glBindTexture(GL_TEXTURE_2D, depthMap);

                // https://emscripten.org/docs/optimizing/Optimizing-WebGL.html 32F
                gl.TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT32F as i32, SHADOW_WIDTH, SHADOW_HEIGHT, 0, gl::DEPTH_COMPONENT, gl::FLOAT, ptr::null());
                //.TexImage2D(GL_TEXTURE_2D, 0, GL_DEPTH_COMPONENT, SHADOW_WIDTH, SHADOW_HEIGHT, 0, GL_DEPTH_COMPONENT, GL_FLOAT, NULL);

                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); //  glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST);
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); //   glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); //     glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
                gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); //     glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);

                gl.BindFramebuffer(gl::FRAMEBUFFER, depth_map_fbo); //     glBindFramebuffer(GL_FRAMEBUFFER, depthMapFBO);
                gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, depth_map, 0);
                //.FramebufferTexture2D(GL_FRAMEBUFFER, GL_DEPTH_ATTACHMENT, GL_TEXTURE_2D, depthMap, 0);

                gl.DrawBuffers(gl::NONE as i32,ptr::null_mut()); //     glDrawBuffer(GL_NONE);
                gl.ReadBuffer(gl::NONE); //     glReadBuffer(GL_NONE);
                gl.BindFramebuffer(gl::FRAMEBUFFER, 0); //     glBindFramebuffer(GL_FRAMEBUFFER, 0);

                gl.UseProgram(shader); //    shader.use();
                gl_int(gl, shader, 0, "diffuseTexture"); //     shader.setInt("diffuseTexture", 0);
                gl_int(gl, shader, 1, "shadowMap"); //     shader.setInt("shadowMap", 1);
            }
            (depth_map_fbo, depth_map)
        };


        let (light_pos, light_space_matrix) = OpenglShadow::get_light_pos(-2.0f32, 4.0, -1.0, 0.0);

        OpenglShadow {
            depth_map_fbo: depth_map_fbo,
            depth_map: depth_map,
            simple_depth_shader: simple_depth_shader,
            shader,
            light_space_matrix: light_space_matrix,
            light_pos: light_pos,
        }
    }
    fn get_light_pos(x:f32, y:f32, z:f32,camera_angle:f32) -> (Vector3<f32>, Matrix4<f32>) {
        let light_pos = vec3(x, y+4.0, z);

        let light_projection = ortho(-10.0f32, 10.0f32, -10.0f32, 10.0f32, OpenglShadow::NEAR_PLANE, OpenglShadow::FAR_PLANE);

        let mut centre = vec3(x,0.0,z);

        let axis = Vector3::new(0.0, 1.0, 0.0).normalize();
        let r: Basis3<_>  = Rotation3::from_axis_angle(axis,Deg(camera_angle));
        let direction = r.rotate_vector(vec3(0.0,0.0,1.0));

        centre = centre + direction * -5.0;


        let light_view = Matrix4::look_at(vec2point(light_pos),
                                          vec2point(centre),
                                          vec3(0.0,1.0,0.0));

        let light_space_matrix = light_projection * light_view;

        return (light_pos, light_space_matrix);
    }
    pub fn update_light_pos(&mut self,x:f32, y:f32, z:f32,camera_angle:f32) {
        let (light_pos, light_space_matrix) = OpenglShadow::get_light_pos(x, y, z, camera_angle);
        self.light_pos = light_pos;
        self.light_space_matrix = light_space_matrix;
    }

    pub fn start_render_shadow(&mut self, gl: &gl::Gl) {
        unsafe {
            gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            gl.Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);

            gl.UseProgram(self.simple_depth_shader);
            gl_matrix4(gl, self.simple_depth_shader, self.light_space_matrix, "lightSpaceMatrix");

            gl.Viewport(0, 0, SHADOW_WIDTH, SHADOW_HEIGHT);
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.depth_map_fbo);
            gl.Clear(gl::DEPTH_BUFFER_BIT);

        }
    }

    pub fn after_rendersceneshadow(&mut self, gl: &gl::Gl) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0); //        glBindFramebuffer(GL_FRAMEBUFFER, 0);
        }
    }
    pub fn before_renderscenenormal(&mut self, gl: &gl::Gl, camera:Vector3<f32>) {
        unsafe {
            // 2. then render scene as normal with shadow mapping (using depth map)
            gl.Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
            gl.Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl.UseProgram(self.shader);

            gl_vec3(gl, self.shader, vec3(camera.x, camera.y, camera.z), "viewPos");
            gl_vec3(gl, self.shader, self.light_pos, "lightPos");
            gl_matrix4(gl, self.shader, self.light_space_matrix, "lightSpaceMatrix");

            gl.ActiveTexture(gl::TEXTURE1);
            gl.BindTexture(gl::TEXTURE_2D, self.depth_map);
        }
    }
}

