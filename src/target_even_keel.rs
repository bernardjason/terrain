use std::fmt;
use std::fmt::Formatter;

use cgmath::{vec3, Vector3};

use crate::plane::{PlaneManoeuvre, TARGET_TIME, TOP_SPEED};

pub struct TargetEvenKeel {
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    clicks: i32,
    satisfied_all_done:bool,
}

impl TargetEvenKeel {
    pub fn new(_target_position:&Vector3<f32>) -> TargetEvenKeel {
        TargetEvenKeel {
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            clicks: 120,
            satisfied_all_done:false
        }
    }
}

impl fmt::Display for TargetEvenKeel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:2.2}  {:2.2}  {:2.2} {} ", self.x_rotation,self.y_rotation,self.z_rotation,self.clicks)
    }
}

impl PlaneManoeuvre for TargetEvenKeel {
    fn update(&mut self, _location:&Vector3<f32>, _target:&Vector3<f32>,pitch: f32, roll: f32, yaw: f32) -> (bool,f32) {
        if self.clicks >= TARGET_TIME {
            println!("**** start TargetEvenKeel  self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                     self.x_rotation, pitch, self.y_rotation, yaw, self.z_rotation, roll);
        }

        self.x_rotation = pitch;
        self.y_rotation = yaw;
        self.z_rotation= roll;

        /*
        if pitch != 0.0 {
           self.x_rotation = 0.0;
        } else if roll != 0.0 {
            self.z_rotation = 0.0;
        } else {
            self.y_rotation = 0.0;
        }
*/

        if self.x_rotation == pitch && self.y_rotation == yaw && self.z_rotation == roll {
            self.satisfied_all_done = true;
            //println!("**** target_even_keel satisfied all done  ");
        } else {
            self.satisfied_all_done = false;
            /*
            println!("**** target_even_keel  self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                     self.x_rotation,pitch,self.y_rotation,yaw, self.z_rotation,roll);

             */
        }
        (false,0.0)
    }
    fn count_down(&mut self,_pitch: f32, _roll: f32, _yaw: f32) -> (bool,bool,f32) {
        self.clicks = self.clicks - 1;
        if self.clicks <= 0 && self.satisfied_all_done {
            println!("target_even_keel all done  ");
            (true,false,0.0)
        } else {
            (false,false,0.0)
        }
    }

    fn rotation(&self) -> Vector3<f32> {
        return vec3(self.x_rotation,self.y_rotation,self.z_rotation)
    }

    fn get_speed(&self) -> f32 {
        TOP_SPEED
    }
}
