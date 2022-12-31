use std::fmt;
use std::fmt::Formatter;

use cgmath::{vec2, vec3, Vector3};

use crate::plane::{degrees, PlaneManoeuvre, TARGET_TIME, calculate_angle, SPEED, TURN_SPEED};

pub struct TargetY {
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    clicks: i32,
    target_position: Vector3<f32>,
    dynamic_target: bool,
    satisfied_all_done: bool,
    speed:f32,
}


impl TargetY {
    pub fn new(target_position: &Vector3<f32>, dynamic_target: bool) -> TargetY {
        TargetY {
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            clicks: TARGET_TIME,
            target_position: target_position.clone(),
            dynamic_target,
            satisfied_all_done: false,
            speed:SPEED,
        }
    }
}

impl fmt::Display for TargetY {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:2.2}  {:2.2}  {:2.2} {} ", self.x_rotation, self.y_rotation, self.z_rotation, self.clicks)
    }
}

impl PlaneManoeuvre for TargetY {
    fn update(&mut self, location: &Vector3<f32>, target: &Vector3<f32>, pitch: f32, roll: f32, yaw: f32) -> (bool,f32) {

        if self.clicks >= TARGET_TIME  {
            println!("**** start target_y (target=x={:2.2} y={:2.2} z={:2.2} ) current_y={:2.2}  self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                        target.x,target.y,target.z,
                     location.y,
                     self.x_rotation, pitch, self.y_rotation, yaw, self.z_rotation, roll);
        }
        let mut target = if self.dynamic_target {
            target.clone()
        } else {
            self.target_position
        };
        if target.y < 1.15 {
            target.y = 1.15;
            self.speed = TURN_SPEED;
        }

        let mut alter_pitch = if location.y < target.y {
            calculate_angle(vec2(target.z - location.z, target.y - location.y))
        } else {
            360.0 - calculate_angle(vec2(target.z - location.z, location.y - target.y))
        };
        if location.z > target.z {
            alter_pitch = 360.0 - alter_pitch;
        }

        let y_rotation = yaw;
        let z_rotation = 0.0;


        self.x_rotation = alter_pitch;
        self.y_rotation = y_rotation;
        self.z_rotation = z_rotation;


        let difference = if location.y < -991.5 {
            0.0
        } else {
            (location.y.abs() - target.y.abs()).abs()
        };

        if difference < 0.2 {
            self.x_rotation = 0.0;
        }
        //if self.x_rotation == pitch && self.y_rotation == yaw && self.z_rotation == roll {
        if self.x_rotation == pitch && difference < 0.15   {
                println!("**** target_y all done difference {} x_rotation={}  pitch={}",difference, self.x_rotation,pitch);
                self.satisfied_all_done = true;
        } else {
                self.satisfied_all_done = false;
        }

        (false,0.0)
    }
    fn count_down(&mut self,pitch: f32, roll: f32, yaw: f32) -> (bool,bool,f32) {
        self.clicks = self.clicks - 1;
        if (self.clicks <= 0 && pitch.round() == 0.0 ) || self.satisfied_all_done {

            //println!("target_y done pitch={},roll={},yaw={}",pitch,roll,yaw);
            println!("target_y done self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                     self.x_rotation, pitch, self.y_rotation, yaw, self.z_rotation, roll);
            (true,false,0.0)
        } else {
            (false,false,0.0)
        }
    }

    fn rotation(&self) -> Vector3<f32> {
        return vec3(self.x_rotation, self.y_rotation, self.z_rotation);
    }

    fn get_speed(&self) -> f32 {
        self.speed
    }
}
