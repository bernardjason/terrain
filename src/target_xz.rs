use std::fmt;
use std::fmt::Formatter;

use cgmath::{vec2, vec3, Vector3, MetricSpace, };

use crate::plane::{PlaneManoeuvre, RADIUS_FROM_TARGET, TARGET_TIME, calculate_angle, TOP_SPEED};
use crate::target_roll_xz::TargetRollXZ;

pub struct TargetXZ {
    x_rotation: f32,
    y_rotation: f32,
    z_rotation: f32,
    clicks: i32,
    target_position:Vector3<f32>,
    dynamic_target:bool,
    satisfied_all_done:bool,
}

impl TargetXZ {
    pub fn new(target_position:&Vector3<f32>,dynamic_target:bool) -> TargetXZ {
        TargetXZ {
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            clicks: TARGET_TIME  ,
            target_position:target_position.clone(),
            dynamic_target,
            satisfied_all_done:false,
        }
    }
}

impl fmt::Display for TargetXZ {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:2.2}  {:2.2}  {:2.2} {} ", self.x_rotation,self.y_rotation,self.z_rotation,self.clicks)
    }
}


impl PlaneManoeuvre for TargetXZ {
    fn update(&mut self, location:&Vector3<f32>, player_pos: &Vector3<f32>, pitch: f32, roll: f32, yaw: f32) -> (bool,f32) {
        let target:Vector3<f32> = if self.dynamic_target {
            player_pos.clone()
        } else {
            self.target_position.clone()
        };
        //let mut y_rotation = self.calculate_angle(vec2( target.x - location.x,target.z - location.z))  ;
        let mut y_rotation = calculate_angle(vec2( location.x - target.x,location.z - target.z))  ;
        y_rotation = TargetRollXZ::correct_the_angle(y_rotation);
        y_rotation = TargetRollXZ::correct_the_angle(y_rotation);

        if self.clicks >= TARGET_TIME  {
            println!("**** start target_xz (target=x={:2.2} y={:2.2} z={:2.2} ) location=(x={:2.2} y={:2.2} z={:2.2})   self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                     player_pos.x,player_pos.y,player_pos.z,
                     location.x,location.y,location.z,
                     self.x_rotation, pitch, self.y_rotation, yaw, self.z_rotation, roll);
            println!("*** ANGLE {} current pitch {}  current yaw {} self.y_roation = {}",y_rotation, pitch,yaw,self.y_rotation);
        }


        self.x_rotation = 0.0;
        self.y_rotation = y_rotation;
        self.z_rotation= 0.0;

        //println!("pitch = {:2.2}   specific {:2.2}  {:2.2} {:2.2} ",pitch,x_rotation,y_rotation,z_rotation);

        let location_xz=vec2(location.x,location.z);
        let target_xz=vec2(target.x,target.z);
        let difference = location_xz.distance2(target_xz);

        //if self.x_rotation == pitch && self.y_rotation == yaw && self.z_rotation == roll {
        if self.y_rotation.round() == yaw.round() {
            self.satisfied_all_done = true;
            println!("**** target_xz all done");
        } else {
            self.satisfied_all_done = false;
            /*
            println!("**** target_xz  self.x_rotation={} pitch={} self.y_rotation={} yaw={} self.z_rotation={} roll={} ",
                     self.x_rotation,pitch,self.y_rotation,yaw, self.z_rotation,roll);

             */
        }

        if difference < RADIUS_FROM_TARGET  {
            println!("**** DONE target_xz {} ",difference);
            self.clicks = 0;
        }
        (false,0.0)
    }
    fn count_down(&mut self,_pitch: f32, _roll: f32, _yaw: f32) -> (bool,bool,f32) {
        self.clicks = self.clicks - 1;
        if self.clicks <= 0 || self.satisfied_all_done {
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
