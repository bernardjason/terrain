use std::ops::{Add, };

use cgmath::{Angle, Deg, Matrix4, MetricSpace, Transform, vec3, Vector2, Vector3, };
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::bullet::Bullet;
use crate::gl;
use crate::gl_helper::{gl_vec3};
use crate::gl_helper::instance_model::ModelInstance;
use crate::gl_helper::model::Model;
use crate::target_even_keel::TargetEvenKeel;
use crate::target_roll_xz::TargetRollXZ;
use crate::target_xz::TargetXZ;
use crate::target_y::TargetY;
use crate::gl_helper::shader::create_shader;
use crate::ground_map::GroundMap;
use crate::special_effects::SpecialEffects;

pub fn degrees(a: f32) -> f32 {
    if a < 0.0 {
        return a + 360.0;
    }
    if a >= 360.0 {
        return a - 360.0;
    }
    a
}

pub struct Plane {
    model: Model,
    pub instances: Vec<PlaneInstance>,
    spinning:f32,
    our_shader:u32,
    //instance:ModelInstance,
    //applied:Matrix4<f32>,
}


pub struct PlaneInstance {
    instance: ModelInstance,
    pub position: Vector3<f32>,
    matrix: Matrix4<f32>,
    applied: Matrix4<f32>,
    speed: f32,
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    flight_plan: Vec<Box<dyn PlaneManoeuvre>>,
    clicks: u128,
    //crash_warning: i128,
}

pub const RADIUS_FROM_TARGET: f32 = 0.1;
pub const TARGET_TIME: i32 = 240;

pub fn calculate_angle(xy: Vector2<f32>) -> f32 {
    let mut atan2: f32 = (Deg::atan2(xy.x, xy.y).0).round();
    //atan2 = atan2 * -1.0;
    if atan2 < 0.0 {
        atan2 = atan2 + 360.0;
    }
    if atan2 >= 360.0 {
        atan2 = atan2 - 360.0;
    }
    atan2
}

pub(crate) trait PlaneManoeuvre {
    fn update(&mut self, location: &Vector3<f32>, target: &Vector3<f32>, pitch: f32, roll: f32, yaw: f32) -> (bool,f32);

    // less 90 so north is 0

    fn count_down(&mut self, pitch: f32, roll: f32, yaw: f32) -> (bool, bool, f32);
    fn rotation(&self) -> Vector3<f32>;
    fn get_speed(&self) -> f32;
}




const PLANE_RADIUS: f32 = 0.1;
const DIFFICULTY_LEVEL: i32 = 0;
const RANGE_TARGET_PLUS_MINUS: f32 = 2.0;

impl PlaneInstance {
    pub fn update(&mut self, player_position: &Vector3<f32>, delta: f32, bullets: &mut Bullet) {
        self.clicks = self.clicks + 1;
        //self.crash_warning = self.crash_warning - 1;
        let vec = vec3(0.0, 0.0, 1.0f32);
        let dir = self.applied.transform_vector(vec);
        self.position -= dir * self.speed * delta;

        self.matrix = Matrix4::<f32>::from_translation(self.position);
        self.matrix = self.matrix * self.applied;
        self.instance.matrix = self.matrix;

        if self.flight_plan.len() == 0 {
            self.decide_on_next_set_of_flight_plans(player_position);
        }
/*
        if self.position.y < 1.0 && self.crash_warning <= 0 {
            println!("***************** CRASH *****************");
            self.flight_plan.clear();
            self.flight_plan.push(Box::new(TargetEvenKeel::new(player_position)));
            let mut target_position = self.position.clone();
            target_position.y = 3.0;
            self.flight_plan.push(Box::new(TargetY::new(&target_position, false)));
            self.crash_warning = 3000;
        }

 */

        let mut pitch = 0.0;
        let mut roll = 0.0;
        let mut yaw = 0.0;
        let top_plan = self.flight_plan.iter_mut().next().unwrap();
        let (rolling_turn,roll_pitch) = top_plan.update(&self.position, player_position, self.x_rotation, self.z_rotation, self.y_rotation);
        let (finished, update_yaw, new_yaw) = top_plan.count_down(self.x_rotation, self.z_rotation, self.y_rotation);

        self.speed = top_plan.get_speed();


        let adjust_pitch = if self.x_rotation.abs() - top_plan.rotation().x > 180.0 {
            -1.0
        } else {
            1.0
        };
        if rolling_turn {
            if self.z_rotation.round() < top_plan.rotation().z.round() {
                roll = 1.0;
            } else if self.z_rotation.round() > top_plan.rotation().z.round() {
                roll = -1.0;
            }
            if roll == 0.0 {
                self.pitch_rotation_x(roll_pitch);
            } else {
                self.roll_rotation_z(roll);
            }
        } else {
            if self.y_rotation.round() < top_plan.rotation().y.round() {
                yaw = 1.0;
            } else if self.y_rotation.round() > top_plan.rotation().y.round() {
                yaw = -1.0;
            } else if self.z_rotation.round() < top_plan.rotation().z.round() {
                roll = 1.0;
            } else if self.z_rotation.round() > top_plan.rotation().z.round() {
                roll = -1.0;
            } else if self.x_rotation.round() < top_plan.rotation().x.round() {
                pitch = adjust_pitch; //1.0;
            } else if self.x_rotation.round() > top_plan.rotation().x.round() {
                pitch = -adjust_pitch; //-1.0;
            }
            self.pitch_rotation_x(pitch);
            self.yaw_rotation_y(yaw);
            self.roll_rotation_z(roll);
        }


        if finished {
            self.flight_plan.remove(0);
            println!("plan completed  x={:2.2}  y={:2.2} z={:2.2} x_rotation={:2.2}   y_rotation={:2.2}   z_rotation={:2.2} ",
                     self.position.x, self.position.y, self.position.z,
                     self.x_rotation, self.y_rotation, self.z_rotation
            );
            if update_yaw {
                print!("WAS x_rotation={:2.2}   y_rotation={:2.2}   z_rotation={:2.2} ", self.x_rotation, self.y_rotation, self.z_rotation);
                println!("Setting yaw to final pitch as spun round now {}", new_yaw);
                self.y_rotation = new_yaw;
                self.x_rotation = 0.0;
                self.z_rotation = 0.0;
                print!("NOW x_rotation={:2.2}   y_rotation={:2.2}   z_rotation={:2.2} ", self.x_rotation, self.y_rotation, self.z_rotation);
            }

            println!();
        }

        if self.clicks % 200 == 0 {
            let (pos, dir) = self.get_bullet_direction();
            if (self.position.y.abs() - player_position.y.abs()).abs() < 0.3 {
                let first_angle = PlaneInstance::angle_between2d(*player_position, pos);
                let future_angle = PlaneInstance::angle_between2d(*player_position, pos - (dir * 3.0));

                if first_angle - future_angle < 1.0 {
                    bullets.new_instance(dir, pos, vec3(0.0, 1.0, 0.0), true);
                }
            }
        }
    }
    fn decide_on_next_set_of_flight_plans(&mut self, player_position: &Vector3<f32>) {
        let mut rng = rand::thread_rng();


        if rng.gen_range(0, 100) >= DIFFICULTY_LEVEL {
            self.flight_plan_target(player_position, &mut rng, true);
        } else {
            self.add_random_target(&mut rng);
            self.add_random_target(&mut rng);
        }
        //self.flight_plan.push(Box::new(TargetEvenKeel::new(player_position)));

        //Box::new(TargetY::new()),
        //Box::new(TargetRollXZ::new()),
        //Box::new(TargetXZ::new()),
    }

    fn add_random_target(&mut self, mut rng: &mut ThreadRng) {
        let mut first_target = vec3(
            rng.gen_range(-RANGE_TARGET_PLUS_MINUS, RANGE_TARGET_PLUS_MINUS) * 3.0,
            rng.gen_range(-RANGE_TARGET_PLUS_MINUS, RANGE_TARGET_PLUS_MINUS),
            rng.gen_range(-RANGE_TARGET_PLUS_MINUS, RANGE_TARGET_PLUS_MINUS) * 3.0,
        ).add(self.position);
        if first_target.y < 1.0 {
            first_target.y = 1.0;
        }
        if first_target.y > 4.0 {
            first_target.y = 4.0;
        }
        self.flight_plan_target(&first_target, &mut rng, false);
    }

    fn flight_plan_target(&mut self, target_position: &Vector3<f32>, rng: &mut ThreadRng, dynamic_target: bool) {
        let distance = self.position.distance2(*target_position);
        let y_diff = distance / (self.position.y.abs() - target_position.y.abs()).abs();
        let z_diff = distance / (self.position.z.abs() - target_position.z.abs()).abs();
        let x_diff = distance / (self.position.x.abs() - target_position.x.abs()).abs();
        if y_diff > z_diff && y_diff > x_diff {
            self.flight_plan.push(Box::new(TargetY::new(target_position, dynamic_target)));
            self.do_either_roll_or_yaw(target_position, rng);
            self.flight_plan.push(Box::new(TargetEvenKeel::new(target_position)));
            //self.flight_plan.push(Box::new(TargetXZ::new(target_position, true)));
        } else {
            self.do_either_roll_or_yaw(target_position, rng);
            self.flight_plan.push(Box::new(TargetY::new(target_position, dynamic_target)));
            self.flight_plan.push(Box::new(TargetEvenKeel::new(target_position)));
            //self.flight_plan.push(Box::new(TargetXZ::new(target_position, true)));
        }
        println!("Distance is {}", distance);
    }

    fn do_either_roll_or_yaw(&mut self, target_position: &Vector3<f32>, rng: &mut ThreadRng) {
        if rng.gen_range(0, 100) >= 0 { // 9999990

            // work out position ahead to give a little time to roll
            let (ahead,dir) = self.get_bullet_direction();
            self.flight_plan.push(Box::new(
                TargetRollXZ::new(target_position, &ahead.add(dir ), self.y_rotation, false)));
            self.flight_plan.push(Box::new(TargetXZ::new(target_position, true)));
        } else {
            self.flight_plan.push(Box::new(TargetXZ::new(target_position, true)));
        }
    }

    fn angle_between2d(a: Vector3<f32>, b: Vector3<f32>) -> f32 {
        let x = a.x - b.x;
        let y = a.z - b.z;
        let answer = Deg::atan2(x, y);
        return answer.0;
    }
    fn get_bullet_direction(&self) -> (Vector3<f32>, Vector3<f32>) {
        let pos = self.position.clone();
        let vec = vec3(0.0, 0.0, 1.0f32);
        return (pos, self.applied.transform_vector(vec));
    }

    fn yaw_rotation_y(&mut self, y: f32) {
        self.y_rotation = degrees(self.y_rotation + y);
        self.applied = self.applied * Matrix4::<f32>::from_angle_y(Deg(y));
    }
    fn pitch_rotation_x(&mut self, x: f32) {
        self.x_rotation = degrees(self.x_rotation + x);
        self.applied = self.applied * Matrix4::<f32>::from_angle_x(Deg(x));
    }
    fn roll_rotation_z(&mut self, z: f32) {
        self.z_rotation = degrees(self.z_rotation + z);
        self.applied = self.applied * Matrix4::<f32>::from_angle_z(Deg(z));
    }
}

pub const SPEED: f32 = 0.25; //0.45;
pub const TOP_SPEED: f32 = 0.45; //0.75;
pub const TURN_SPEED: f32 = 0.25;

impl Plane {
    pub fn new(gl: &gl::Gl) -> Plane {
        let model = Model::new(gl, "resources/simpleplane/simpleplane.obj", "resources/simpleplane/simpleplane.png");
        let our_shader = create_shader(&gl, PLANE_VS, PLANE_FS, None);
        Plane {
            model,
            instances: vec![],
            spinning:0.0,
            our_shader,
        }
    }


    pub fn new_instance(&mut self, gl: &gl::Gl, position: Vector3<f32>) {
        let mut instance = ModelInstance::new(gl, self.model.clone(), 0.1, None);
        instance.matrix = Matrix4::<f32>::from_translation(position);

        let plan: Vec<Box<dyn PlaneManoeuvre>> = vec![
            //Box::new(TargetY::new(&vec3(0.0,1.0,0.0),false))
            //Box::new(TargetRollXZ::new()),
            //Box::new(TargetXZ::new()),
        ];
        let new_plane = PlaneInstance {
            instance,
            matrix: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
            applied: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
            position,
            speed: SPEED,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            flight_plan: plan,
            clicks: 0,
        };

        self.instances.push(new_plane);
    }

    pub fn update(&mut self, player_position: Vector3<f32>, delta: f32, bullets: &mut Bullet, ground_map: &GroundMap, special_effects: &mut SpecialEffects) {
        let mut remove: Vec<usize> = vec![];
        for i in 0..self.instances.len() {
            self.instances[i].update(&player_position, delta, bullets);
            if player_position.distance2(self.instances[i].position) > 400.0 {
                println!("Too far away remove plane");
                remove.insert(0, i);
            } else if ground_map.height(self.instances[i].position.x, self.instances[i].position.z) >= self.instances[i].position.y - 0.1 {
                special_effects.explosion(self.instances[i].position);
                println!("CRASH!!!!!!!!!!!!!!!!!!!" );
                remove.insert(0, i);
            }
        }
        for i in remove {
            self.instances.remove(i);
        }
    }


    pub fn been_hit(&mut self, other: Vector3<f32>, special_effects: &mut SpecialEffects) -> bool {
        let mut hit = false;
        let mut remove: Vec<usize> = vec![];
        for i in 0..self.instances.len() {
            let instance = &self.instances[i];
            if instance.position.distance2(other) < PLANE_RADIUS {
                remove.insert(0, i);
                special_effects.explosion(instance.position);
            }
        }
        for i in remove {
            self.instances.remove(i);
            hit = true;
        }
        hit
    }


    pub(crate) fn render(&mut self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>, our_shader: u32,shadow:bool) {
        //self.instance.matrix = self.instance.matrix * self.applied;
        //self.instance.matrix = self.instance.matrix * Matrix4::from_angle_y(Deg(1.75));
        unsafe {
            if shadow {
                gl.UseProgram(our_shader);
            } else {
                gl.UseProgram(self.our_shader);
                gl_vec3(gl,self.our_shader,vec3(self.spinning%6.0,0.0,0.0),"spinning");
            }



            for i in self.instances.iter() {
                if shadow {
                    i.instance.render(gl, view, projection, our_shader, false);
                } else {
                    i.instance.render(gl, view, projection, self.our_shader, false);
                }
            }
            self.spinning=self.spinning+1.0;
        }
    }
}

pub const PLANE_FS: &str = "#version 300 es
precision lowp float;
out vec4 FragColor;

in vec2 TexCoord;
in vec3 spin;

uniform sampler2D texture0;

void main()
{
    FragColor = texture(texture0, TexCoord);
    if ( TexCoord.x < 0.3 && TexCoord.y > 0.6 ) {
        if ( FragColor.x > 0.7 && FragColor.y > 0.7 && FragColor.z < 0.1  ) {
            if ( spin.x  >= 3.0 ) {
                FragColor = vec4(0,0,0,1);
            } else {
                FragColor = vec4(0,0,0,0);
            }
        } else if ( FragColor.x > 0.7 ) {
                if ( spin.x  <= 3.0 ) {
                        FragColor = vec4(0,0,0,1);
                } else {
                    FragColor = vec4(0,0,0,0);
                }
        } else {
            FragColor = vec4(0,0,0,0);
        }
    }
}
";
pub const PLANE_VS: &str = "#version 300 es
precision lowp float;

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

out vec2 TexCoord;
out vec3 spin;

uniform mat4 model;
uniform vec3 spinning;
uniform mat4 view;
uniform mat4 projection;

void main()
{
        gl_Position = projection * view * model * vec4(aPos, 1.0f);
        TexCoord = vec2(aTexCoord.x, aTexCoord.y);
        spin = spinning;
}
";
