#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use std::f32;
use cgmath::{vec3, Zero, InnerSpace};


type Point3 = cgmath::Point3<f32>;
type Vector3 = cgmath::Vector3<f32>;
type Matrix4 = cgmath::Matrix4<f32>;

// Defines several possible options for camera movement. Used as abstraction to stay away from window-system specific input methods

// Default camera values
pub const CAMERA_ANGLE_FUDGE:f32 = 90.0;
const YAW: f32 = -CAMERA_ANGLE_FUDGE;
const ROLL: f32 = 0.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 0.125;
pub const PERSPECTIVE_ANGLE: f32 = 45.0;
pub const ZOOM: f32 = 45.0;

pub struct Flying_Camera {
    // Flying_Camera Attributes
    pub Position: Point3,
    pub PreviousPosition:Point3,
    pub Front: Vector3,
    pub direction:Vector3,
    pub Up: Vector3,
    pub Right: Vector3,
    pub WorldUp: Vector3,
    // Euler Angles
    pub Yaw: f32,
    pub Roll: f32,
    pub Pitch: f32,
    // Flying_Camera options
    pub MovementSpeed: f32,
}

impl Default for Flying_Camera {
    fn default() -> Flying_Camera {
        let mut camera = Flying_Camera {
            Position: Point3::new(0.0, 0.0, 0.0),
            PreviousPosition: Point3::new(0.0, 0.0, 0.0),
            Front: vec3(0.0, 0.0, -1.0),
            direction:vec3(0.0,0.0,-1.0),
            Up: Vector3::zero(), // initialized later
            Right: Vector3::zero(), // initialized later
            WorldUp: Vector3::unit_y(),
            Yaw: YAW,
            Pitch: PITCH,
            Roll:ROLL,
            MovementSpeed: SPEED,
        };
        camera.updateFlying_CameraVectors();
        camera
    }
}

impl Flying_Camera {
    /// Returns the view matrix calculated using Eular Angles and the LookAt Matrix
    pub fn GetViewMatrix(&self) -> Matrix4 {
        //println!("GetMAtrix {},{},{}",self.Up.x,self.Up.y,self.Up.z);
        Matrix4::look_at(self.Position, self.Position + self.Front, self.Up)
    }

    pub fn getPositionAndForwardTo(&self, deltaTime:f32) -> (cgmath::Point3<f32>, cgmath::Point3<f32>) {
        let velocity = self.MovementSpeed * deltaTime;

        let here = self.Position + self.direction * velocity;
        let next = self.Position + self.direction * velocity*15.0;

        return (here,next);
    }

    pub fn save_position(&mut self) {
        self.PreviousPosition = self.Position;
    }
    pub fn rollback(&mut self) {
        self.Position = self.PreviousPosition;
    }


    /// Calculates the front vector from the Flying_Camera's (updated) Eular Angles
    pub fn updateFlying_CameraVectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3 {
            x: self.Yaw.to_radians().cos() * self.Pitch.to_radians().cos() ,
            y: self.Pitch.to_radians().sin(),
            z: self.Yaw.to_radians().sin() * self.Pitch.to_radians().cos() ,
        };
        self.Front = front.normalize();
        let direction = Vector3 {
            x: self.Yaw.to_radians().cos(),
            y: 0.0,
            z: self.Yaw.to_radians().sin()
        };
        self.direction = direction.normalize();

        // Also re-calculate the Right and Up vector
        self.Right = self.Front.cross(self.WorldUp).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.Up = self.Right.cross(self.Front).normalize();
    }

    pub fn updateCameraVectors(&mut self) {
        // Calculate the new Front vector
        let front = Vector3 {
            x: self.Yaw.to_radians().cos() * self.Pitch.to_radians().cos(),
            y: self.Pitch.to_radians().sin(),
            z: self.Yaw.to_radians().sin() * self.Pitch.to_radians().cos(),
        };
        self.Front = front.normalize();
        // Also re-calculate the Right and Up vector
        self.Right = self.Front.cross(self.WorldUp).normalize(); // Normalize the vectors, because their length gets closer to 0 the more you look up or down which results in slower movement.
        self.Up = self.Right.cross(self.Front).normalize();
    }
    pub fn lookAt(&self,here:Vector3) -> Matrix4 {
        Matrix4::look_at_dir(self.Position, here, self.Up)

    }
}
