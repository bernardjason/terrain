use std::fmt;
use std::fmt::Formatter;

use cgmath::{vec2, vec3, Vector3};
use rand::Rng;

use crate::plane::{PlaneManoeuvre, TARGET_TIME, calculate_angle, TURN_SPEED};

pub struct TargetRollXZ {
    original_angle:f32,
    target_yaw_pitch:f32,
    final_yaw_pitch:f32,
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    clicks: i32,
    roll_left: bool,
    pitch_acheived: bool,
    satisfied_all_done: bool,
}

impl TargetRollXZ {
    pub fn new(target_position: &Vector3<f32>, location:&Vector3<f32>,yaw:f32, dynamic_target: bool) -> TargetRollXZ {
        if dynamic_target {
            panic!("roll turn is intended to pick a target and turn to it, i dont want it rolling forever")
        }
        let mut rng = rand::thread_rng();
        let roll_left = if rng.gen_range(0, 10) > 50 {
            false
        } else {
            true
        };
        let original_angle = TargetRollXZ::correct_the_angle(
            TargetRollXZ::get_pitch_we_want(location, target_position));

        let final_yaw_pitch = TargetRollXZ::correct_the_angle(original_angle - yaw);
        let target_yaw_pitch = if roll_left {
            final_yaw_pitch
        } else {
            final_yaw_pitch
        };

        TargetRollXZ {
            target_yaw_pitch,
            final_yaw_pitch,
            original_angle,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            clicks: TARGET_TIME,
            roll_left:false,
            pitch_acheived: false,
            satisfied_all_done: false,
        }
    }

}

impl fmt::Display for TargetRollXZ {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:2.2}  {:2.2}  {:2.2} {} ", self.x_rotation, self.y_rotation, self.z_rotation, self.clicks)
    }
}

impl PlaneManoeuvre for TargetRollXZ {
    fn update(&mut self, location: &Vector3<f32>, target: &Vector3<f32>, pitch: f32, mut roll: f32, yaw: f32) -> (bool,f32) {


        if self.clicks == TARGET_TIME  {
            println!("**** start TargetRollXZ (target=x={:2.2} y={:2.2} z={:2.2} )  location=(x={:2.2} y={:2.2} z={:2.2})    self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                     target.x,target.y,target.z,
                     location.x,location.y,location.z,
                     self.x_rotation, pitch, self.y_rotation, yaw, self.z_rotation, roll);

            println!("*** ANGLE target={} rolled_yaw_pitch={} current pitch {}  current yaw {} self.y_roation = {}",self.final_yaw_pitch,self.target_yaw_pitch, pitch,yaw,self.y_rotation);
        }
        if self.target_yaw_pitch == TargetRollXZ::correct_the_angle(pitch ) {
            self.pitch_acheived = true;
            //println!("*** DONE !!!! ANGLE {} current pitch {}  current yaw {} self.y_roation = {}",self.target_yaw_pitch, pitch,yaw,self.y_rotation);
        } else {
            //println!("*** WAITING !!!! ANGLE {} current pitch {}  current yaw {} self.y_roation = {}",self.target_yaw_pitch, pitch,yaw,self.y_rotation);
        }

        let mut roll_pitch:f32 = 0.0;

        if self.pitch_acheived == false {
            //let (rolled,z_rotation) = self.roll_left(&mut roll, );
            let (rolled,z_rotation) = if self.roll_left {
                self.roll_left(&mut roll)
            } else {
                self.roll_right(&mut roll)
            };

            self.z_rotation = z_rotation;

            if rolled {
                roll_pitch = if self.roll_left {
                    1.0
                } else {
                    -1.0
                };
            }


        } else {
            self.z_rotation = 0.0;
        }

        if self.pitch_acheived && roll.round() == 0.0 {
            self.satisfied_all_done = true;
            println!("**** target_roll_xz satisfied all done  ");
        } else {
            self.satisfied_all_done = false;
        }

        (true,roll_pitch)
        //println!("specific {:2.2}  {:2.2} {:2.2} ",x_rotation,y_rotation,z_rotation);
    }
    fn count_down(&mut self, pitch: f32, roll: f32, _yaw: f32) -> (bool, bool, f32) {
        self.clicks = self.clicks - 1;
        //if self.clicks <= 0 && self.satisfied_all_done == true {
        if self.satisfied_all_done == true {
            println!("*** target_roll_xz done roll={} pitch={}", roll, pitch);
            if roll != 0.0 && pitch != 0.0 { panic!("Why isnt it 0"); }
            (true, true, self.original_angle)
        } else {
            (false, false, 0.0)
        }
    }

    fn rotation(&self) -> Vector3<f32> {
        return vec3(self.x_rotation, self.y_rotation, self.z_rotation);
    }

    fn get_speed(&self) -> f32 {
        TURN_SPEED
    }
}

impl TargetRollXZ {
    fn get_pitch_we_want(location: &Vector3<f32>, target: &Vector3<f32>,) ->f32  {
        calculate_angle(vec2( location.x - target.x,location.z - target.z))
    }

    fn roll_right(&mut self, roll: &f32, ) -> (bool,f32)  {
        let z_rotation: f32 = 90.0;
        let compare_roll = TargetRollXZ::get_compare_roll(roll);
        if compare_roll.round() == z_rotation.round() {
            (true,*roll)
        } else {
            (false,z_rotation)
        }
    }
    fn roll_left(&mut self, roll: &f32, ) -> (bool,f32)  {
        let z_rotation: f32 = -90.0;
        let compare_roll = TargetRollXZ::get_compare_roll(roll);
        if compare_roll.round() == z_rotation.round() {
            (true,*roll)
        } else {
            (false,z_rotation)
        }
    }

    fn get_compare_roll(roll: &f32) -> f32 {
        let compare_roll = if *roll > 180.0 {
            *roll - 360.0
        } else {
            *roll
        };
        compare_roll
    }



    pub(crate) fn correct_the_angle(mut atan2: f32) ->f32 {
        if atan2 < 0.0 {
            atan2 = atan2 + 360.0;
        }
        if atan2 >= 360.0 {
            atan2 = atan2 - 360.0;
        }
        return atan2.round()
    }

}
