use std::collections::VecDeque;

use cgmath::{Angle, Deg, InnerSpace, Matrix4, MetricSpace, perspective, Transform, vec3, Vector3, };

use crate::{FAR, gl, HEIGHT, WIDTH};
use crate::alienship::AlienShip;
use crate::flying_camera::{Flying_Camera, ZOOM};

const SPEED_UP: f32 = 1.0;

pub struct Player {
    alienship: AlienShip,
    pitch: f32,
    roll: f32,
    speed: f32,
    camera_queue: VecDeque<Vector3<f32>>,
    pub position: Vector3<f32>,
    old_matrix: Matrix4<f32>,
    old_position: Vector3<f32>,
    matrix: Matrix4<f32>,
    applied: Matrix4<f32>,
    camera_dir: Vector3<f32>,
    pub(crate) been_hit: i32,
    pub player_reset: bool,
    player_reset_countdown: i128,
}


impl Player {
    pub fn new(gl: &gl::Gl, position: Vector3<f32>) -> Player {
        let alienship = AlienShip::new(gl, position);

        Player {
            alienship,
            pitch: 0.0,
            roll: 0.0,
            speed: 0.0,
            camera_queue: VecDeque::new(),
            position,
            old_matrix: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
            old_position: vec3(0.0, 0.0, 0.0),
            matrix: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
            applied: Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0)),
            camera_dir: vec3(0.0, 0.0, 1.0),
            been_hit: 0,
            player_reset: false,
            player_reset_countdown: 0,
        }
    }

    pub fn been_hit(&mut self, location: Vector3<f32>) -> bool {
        if !self.player_reset {
            if location.distance2(self.position) < 0.3 {
                self.been_hit = self.been_hit + 1;
                return true;
            }
        }
        return false;
    }
    pub fn debug_positiom(&mut self, one: bool, two: bool, three: bool, four: bool) {
        if one { self.position = vec3(-4.0, 4.0, -4.0); }
        if two { self.position = vec3(4.0, 4.0, -4.0); }
        if three { self.position = vec3(-4.0, 4.0, 4.0); }
        if four { self.position = vec3(4.0, 4.0, 4.0); }
    }
    pub fn update_player(&mut self) {
        if !self.player_reset {
            self.do_update();
        } else {
            self.player_reset_countdown = self.player_reset_countdown - 1;
            if self.player_reset_countdown <= 0 {
                self.position.y = 4.0;
                self.reset()
            }
        }
    }

    pub fn reset(&mut self) {
        self.player_reset = false;
        self.speed=0.0;
        self.applied = Matrix4::<f32>::from_translation(vec3(0.0, 0.0, 0.0));
        self.pitch=0.0;
        self.roll=0.0;
        self.camera_queue.clear();
        self.do_update()
    }

    fn do_update(&mut self) {
        self.old_matrix = self.matrix;
        self.old_position = self.position;
        self.matrix = Matrix4::<f32>::from_translation(self.position);
        self.matrix = self.matrix * Matrix4::from_scale(0.03);
        self.matrix = self.matrix * self.applied;
    }
    pub fn rollback(&mut self) {
        println!("************* ROLLBACK ********************");
        self.matrix = self.old_matrix;
        self.position = self.old_position;
        self.do_update();
        self.player_reset = true;
        self.player_reset_countdown = 60;
    }
    pub fn process_keyboard_pitch(&mut self, rotate: f32, delta: f32) {
        self.pitch += rotate*delta;
        self.pitch = Player::wrap_around_angle(self.pitch);
        self.applied = self.applied * Matrix4::<f32>::from_angle_x(Deg(-rotate*delta));
    }
    pub fn process_keyboard_roll(&mut self, rotate: f32, delta: f32) {
        self.roll += rotate * delta;
        self.roll = Player::wrap_around_angle(self.roll);
        self.applied = self.applied * Matrix4::<f32>::from_angle_z(Deg(-rotate * delta));
    }
    pub fn queue_camera_for_later(&mut self) {
        let dir = vec3(0.0, 0.0, 1.0f32);
        let dir = self.applied.transform_vector(dir);
        self.camera_queue.push_back(dir);
    }
    pub fn forward_pressed(&mut self, speed: f32) {
        self.speed = self.speed + speed;
        if self.speed >= 2000.0 {
            self.speed = 2000.0;
        }
    }
    pub fn forward(&mut self, direction: f32, delta_time: f32) -> Vector3<f32> {
        if !self.player_reset {
            let vec = vec3(0.0, 0.0, 1.0f32);
            let dir = self.applied.transform_vector(vec);

            if direction < 0.0 {
                self.position += dir * self.speed * delta_time * SPEED_UP;
            } else {
                self.position -= dir * self.speed * delta_time * SPEED_UP;
            }
            return dir;
        }
        return vec3(0.0, 0.0, 0.0);
    }
    pub fn slow_down_resistance(&mut self) {
        self.speed = self.speed * 0.95;
        if self.speed <= 2.0 {
            self.speed = 2.0
        }
    }
    pub fn update_camera(&mut self) {
        if self.camera_queue.len() > 0 {
            self.camera_dir = self.camera_queue.pop_front().unwrap();
        } else {
            self.camera_dir = vec3(0.0, 0.0, 1.0f32);
            self.camera_dir = self.applied.transform_vector(self.camera_dir);
        }
        if self.camera_queue.len() > 30 {
            while self.camera_queue.len() > 30 {
                self.camera_queue.pop_front();
            }
        }
        //println!("{} {:2.2},{:2.2}",rotation.0 as i32,self.camera_dir.x,self.camera_dir.z);
    }
    pub fn rotation_around_ship(&self) -> f32 {
        let mut rotation = Deg::atan2(self.camera_dir.x, self.camera_dir.z).0;
        if rotation < 0.0 {
            rotation = 360.0 + rotation;
        }
        360.0 - rotation
    }

    pub fn projection_view_camera(&self, mut camera: &mut Flying_Camera) -> (Matrix4<f32>, Matrix4<f32>) {
        let mut move_camera_here = self.position.clone();
        let dir = self.camera_dir * 2.0;
        move_camera_here += dir;
        camera.Position.x = move_camera_here.x;
        camera.Position.y = move_camera_here.y;
        camera.Position.z = move_camera_here.z;
        camera.updateCameraVectors();

        let view = camera.lookAt(dir * -1.0);
        let projection: Matrix4<f32> = perspective(Deg(ZOOM),
                                                   WIDTH as f32 / HEIGHT as f32, 0.1, FAR);
        (view, projection)
    }

    pub fn get_bullet_direction(&self) -> (Vector3<f32>, Vector3<f32>) {
        let pos = self.position.clone();


        let vec = vec3(0.0, 0.0, 1.0f32);
        return (pos, self.applied.transform_vector(vec));
    }


    pub(crate) fn render(&mut self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>, our_shader: u32) {
        self.alienship.instance.matrix = self.matrix;
        self.alienship.render(gl, view, projection, our_shader);
    }

    fn wrap_around_angle(a: f32) -> f32 {
        if a >= 360.0 {
            return a - 360.0;
        }
        if a < 0.0 {
            return a + 360.0;
        }

        return a;
    }
}