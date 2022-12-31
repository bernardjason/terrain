use cgmath::Matrix4;

use crate::gl;
use crate::gl_helper::texture::create_texture_png;
use crate::ground_cell::{GroundCell, SCALE};
use crate::pickup::Pickup;
use crate::generate_landscape::GenerateLandscape;

pub const IMAGE_FILE: &str = "resources/multi-colour.png";

pub struct GroundMap {
    ground_texture:u32,
    ground_cells:Vec<GroundCell>,
    display_map:Vec<Vec<i32>>,
    landscape:GenerateLandscape,
}
pub const TOTAL_VERTICES_ONE_SIDE:i32 = 16;



fn remove(to_remove:Vec<i32> , original : Vec<i32> ) -> Vec<i32> {
    let mut dest = vec![];
    for pair in original.chunks(2) {
        let mut add =true;
        for to_remove_pair in to_remove.chunks(2) {
            if to_remove_pair[0] == pair[0] && to_remove_pair[1] == pair[1] {
                add=false;
            }
        }
        if add {
            let mut a = vec![pair[0], pair[1]];
            dest.append(&mut a);
        }
    }
    dest
}

impl GroundMap {

    pub fn new(gl: &gl::Gl) -> GroundMap {
        let texture = create_texture_png(&gl, IMAGE_FILE);

        let row0 =vec![ -2,-2,  -1,-2,  0,-2,  1,-2,  2,-2];
        let row1 =vec![-2, -1, -1, -1, 0, -1, 1, -1, 2, -1];
        let row2 =vec![ -2,0,  -1,0,  0,0,  1,0,  2,0];
        let row3 =vec![ -2,1,  -1,1,  0,1,  1,1,  2,1];
        let row4 =vec![ -2,2,  -1,2,  0,2,  1,2,  2,2];
        let col0=vec![-2,-2,  -2,-1, -2,0,  -2,1,  -2,2];
        let col1=vec![-1,-2,  -1,-1, -1,0,  -1,1,  -1,2];
        //let col2=vec![0,-2,  0,-1, 0,0,  0,1,  0,2];
        let col3=vec![1,-2,  1,-1, 1,0,  1,1, 1,2];
        let col4=vec![2,-2,  2,-1, 2,0,  2,1,  2,2];

        let all:Vec<i32> = [row0.clone(),row1.clone(),row2.clone(),row3.clone(),row4.clone()].concat();

        let mut display_map = vec![vec![0];8];

        /*
        display_map[0] = remove([row3.clone(),row4.clone()].concat(),all.clone()) ;
        display_map[1] = remove(vec![-2,-1,-2,0,-2,1,-2,2,  -1,-1,-1,0,-1,1], all.clone());
        display_map[2] = remove([col0.clone(),col1.clone()].concat(), all.clone());
        display_map[3] = remove(vec![-2,-2,-1,-2,0,-2,1,-2,    -2,-1,-1,-1,0,-1,  -2,0,-1,0, -2,1,],all.clone() );
        display_map[4] = remove([row0.clone(),row1.clone()].concat(),all.clone() );
        display_map[5] = remove(vec![-1,-2,0,-2,1,-2,2,-2,   0,-1,1,-1,2,-1,     1,0,2,0,   2,1],all.clone() );
        display_map[6] = remove([col3,col4].concat(),all.clone() );
        display_map[7] = remove(vec![2,-1,  1,0,2,0,   0,1,1,1,2,1,     -1,2,0,2,1,2,2,2  ],all.clone() );
         */
        // COULD TUNE THIS but kept getting occasional blank
        display_map[0] = all.clone();
        display_map[1] = all.clone();
        display_map[2] = all.clone();
        display_map[3] = all.clone();
        display_map[4] = all.clone();
        display_map[5] = all.clone();
        display_map[6] = all.clone();
        display_map[7] = all.clone();



        let mut ground_map = GroundMap{
            ground_texture:texture,
            ground_cells:vec![],
            display_map:display_map,
            landscape:GenerateLandscape::new(),
        };
        //ground_map.landscape.test_it();

        ground_map
    }
/*
    pub fn xupdate(&mut self,gl:&gl::Gl,position_x:f32,position_z:f32,mut rotation_y:f32) -> usize {
        self.ground_cells.clear();
        let cell_x = (position_x / TOTAL_VERTICES_ONE_SIDE as f32).round();
        let cell_z = (position_z / TOTAL_VERTICES_ONE_SIDE as f32).round();

        rotation_y = (rotation_y + CAMERA_ANGLE_FUDGE) % 360.0;

        let rotation_y_normalized =
            (if rotation_y < 0.0 { rotation_y % 360.0 + 360.0 } else { rotation_y % 360.0 } / 45.0) as usize;

        for add_cell_x in -3..4 {
            for add_cell_z in -3..4 {
                let centre_x = (add_cell_x as f32 + cell_x) * TOTAL_VERTICES_ONE_SIDE as f32;
                let centre_z = (add_cell_z as f32 + cell_z) * TOTAL_VERTICES_ONE_SIDE as f32;
                let cell = GroundCell::new(gl, self.ground_texture, centre_x, centre_z, centre_x as i32, centre_z as i32, &mut self.landscape);
                self.ground_cells.push(cell);
            }
        }
        rotation_y_normalized
    }
    pub fn _update(&mut self,gl:&gl::Gl,position_x:f32,position_z:f32,mut rotation_y:f32) -> usize {
        0
    }
*/

    pub fn update(&mut self,gl:&gl::Gl,position_x:f32,position_z:f32,mut rotation_y:f32,pickups:&mut Pickup) -> usize {
        for cell in self.ground_cells.iter() {
            cell.free(gl);
        }
        self.ground_cells.clear();
        let cell_x = (position_x / (TOTAL_VERTICES_ONE_SIDE as f32 * SCALE)).round();
        let cell_z = (position_z / (TOTAL_VERTICES_ONE_SIDE as f32 * SCALE)).round();

        //rotation_y=(rotation_y + CAMERA_ANGLE_FUDGE) % 360.0;
        rotation_y=(rotation_y ) % 360.0;

        let rotation_y_normalized =
            ( if rotation_y < 0.0 { rotation_y%360.0 + 360.0 } else { rotation_y%360.0} / 45.0 ) as usize;

        let to_display = &self.display_map[rotation_y_normalized ];
        //to_display.windows(2).inspect(|d| { print!("x={},y={}     ",d[0],d[1])  }).collect::<Vec<_>>();
        //println!();
        for i in (0..to_display.len() ).step_by(2) {
            let x = to_display[i];
            let z = to_display[i+1];
            let centre_x = (x as f32 + cell_x) * TOTAL_VERTICES_ONE_SIDE as f32 * SCALE;
            let centre_z = (z as f32  + cell_z)* TOTAL_VERTICES_ONE_SIDE as f32  * SCALE;
            let cell = GroundCell::new(gl, self.ground_texture, centre_x, centre_z, centre_x as i32, centre_z  as i32,
                                       &mut self.landscape,pickups);
            self.ground_cells.push(cell);

        }

        rotation_y_normalized
    }
    pub fn height(&self, x:f32, z:f32) -> f32 {
        self.landscape.pseudo_random((x ) as f64, (z ) as f64) as f32
    }



    pub fn render(&self, gl: &gl::Gl, view: &Matrix4<f32>, projection: &Matrix4<f32>, our_shader:u32) {
        for cell in self.ground_cells.iter() {
            cell.render(gl,view,projection,our_shader);
        }

    }

}