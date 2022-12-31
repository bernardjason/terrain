use cgmath::{Matrix4, vec3, Vector3, Zero};

use crate::{ gl, get_start_time, output_elapsed};
use crate::cube::Cube;
use rand::Rng;
use rand::prelude::ThreadRng;
use crate::gl_helper::gl_vec3;
use crate::gl_helper::shader::create_shader;

pub struct SpecialEffects {
    cube: Cube,
    pub instances: Vec<SpecialInstance>,
    colours:Vec<Vector3<f32>>,
    our_shader:u32,
}

pub struct SpecialInstance {
    pub position: Vector3<f32>,
    direction: Vector3<f32>,
    scale:f32,
    ticks: i32,
    speed:f32,
    tex_index:usize,
}

impl SpecialEffects {
    pub fn new(gl: &gl::Gl) -> SpecialEffects {
        let start = get_start_time();
        let cube = Cube::new(&gl, "", vec3(0.001, 0.001, 0.001), 1.0);
        //let yellow = create_texture_png(&gl, "resources/multi-colour.png");
        //let purple = create_texture_png(&gl, "resources/multi-colour.png");
        let our_shader = create_shader(&gl, EFFECTS_VS, EFFECTS_FS, None);

        output_elapsed(start,"Time elapsed in special effects new ()");
        SpecialEffects {
            cube,
            instances: Vec::new(),
            colours: vec![ vec3(1.0,0.0,0.0) , vec3(1.0,1.0,0.0) , vec3(1.0,0.0,1.0)],
            our_shader,
        }
    }

    pub fn explosion(&mut self, position: Vector3<f32>) {
        let mut rng = rand::thread_rng();
        //position.y = position.y - 0.1;
        self.create_explosion_block(position, rng,10.0,8,);
        for _i in 0..30 {
            let scale= rng.gen_range(10.0, 30.0);
            let ticks= rng.gen_range(20, 30);
            self.create_explosion_block(position, rng,scale,ticks,);
        }
    }

    fn create_explosion_block(&mut self, position: Vector3<f32>, mut rng: ThreadRng,scale:f32,ticks:i32,) {
        let direction: Vector3<f32> = vec3(
            rng.gen_range(-0.2, 0.2),
            rng.gen_range(0.2, 0.7),
            rng.gen_range(-0.2, 0.2));


        let instance = SpecialInstance {
            direction,
            position,
            ticks,
            scale,
            speed: rng.gen_range(0.5, 1.0),
            tex_index: 0,
        };
        self.instances.push(instance);
    }


}

impl SpecialEffects {
    pub fn update(&mut self, delta: f32,) {
        for i in (0..self.instances.len()).rev() {
            let change = self.instances.get_mut(i).unwrap();
            change.tex_index = change.tex_index +1;

            if change.speed.is_zero() {
                let mut rng = rand::thread_rng();
              change.scale =  change.scale * rng.gen_range(0.4,0.7);

            } else {
                change.position += change.direction * delta * change.speed;
            }

            change.ticks = change.ticks - 1;
            if change.ticks <= 0   {
                self.instances.remove(i);
            }
            //change.matrix = Matrix4::<f32>::from_translation(change.collision.position);
        }
    }
    pub fn render(&mut self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>,_our_shader:u32) {
        unsafe {
            gl.UseProgram(self.our_shader);
        }
        for i in self.instances.iter_mut() {
            let scale =  Matrix4::<f32>::from_scale(i.scale);
            let matrix = Matrix4::<f32>::from_translation(i.position) * scale;
            let colour = self.colours[i.tex_index%self.colours.len()];
            gl_vec3(gl,self.our_shader,vec3(colour.x,colour.y,colour.z),"colour");
            i.tex_index=i.tex_index+1;
            self.cube.render(gl, &matrix, view, projection,self.our_shader,self.cube.texture);
        }
    }
}

pub const EFFECTS_FS: &str = "#version 300 es
precision lowp float;
out vec4 FragColor;

in vec2 TexCoord;
in vec3 use_colour;

uniform sampler2D texture0;

void main()
{
    FragColor = vec4(use_colour.x,use_colour.y,use_colour.z,1.0);
}
";
pub const EFFECTS_VS: &str = "#version 300 es
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
        TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        use_colour=colour;
}
";