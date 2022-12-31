use crate::ground_cell::SEA;


pub struct Landscape {
    pub(crate) max:f64,
    pub(crate) min:f64,
    //perlin:Perlin
}
 impl Landscape {
     pub fn new() -> Landscape {

         //let perlin = Perlin::new(2);

         Landscape {
             min:0.0,
             max:0.0,
             //perlin
         }
     }
     pub fn test_it(&mut self) {

         let val =  Landscape::noise(3.14 , 42.0, 7.0    )  ;
         println!("BASE {}",val);

         let mut max = 0.0;
         let mut min = 0.0;
        for xx in -10..11 {
            for zz in -10..11 {
                let height = self.pseudo_random(xx as f64, zz as f64);
                print!("{}   ",height);
                if height > max {
                    max=height
                }
                if height < min {
                    min=height
                }
            }
            println!("\n");
        }
         println!("TEST max {} min {}",max,min);
     }

     pub fn pseudo_random(&mut self, x:f64, z:f64 ) -> f64 {
         //let val = ( Landscape::noise(x as f64    ,(x+z) as f64 / 0.05  ,z as f64   )  ) as f32 ;
         let y = P[  (x.abs() * z.abs()).abs() as usize % P.len() ] as f64;
         let val =  Landscape::noise(x * 9.123 , y ,  z  * 9.123   ) ;

         //println!("{},{}         {}",x,z,val);
         if val  < self.min  {
             self.min = val;
         }
         if val > self.max  {
             self.max = val;
         }

         val
     }

     fn noise(x: f64, y: f64, z: f64) -> f64 {
         let x0 = x.floor() as usize & 255;
         let y0 = y.floor() as usize & 255;
         let z0 = z.floor() as usize & 255;

         let x = x - x.floor();
         let y = y - y.floor();
         let z = z - z.floor();

         let u = Landscape::fade(x);
         let v = Landscape::fade(y);
         let w = Landscape::fade(z);

         let a = P[x0] + y0;
         let aa = P[a] + z0;
         let ab = P[a + 1] + z0;
         let b = P[x0 + 1] + y0;
         let ba = P[b] + z0;
         let bb = P[b + 1] + z0;

         let mut height = Landscape::lerp(w,
                     Landscape::lerp(v, Landscape::lerp(u, Landscape::grad(P[aa], x    , y    , z),
                                  Landscape::grad(P[ba], x-1.0, y    , z)),
                          Landscape::lerp(u, Landscape::grad(P[ab], x    , y-1.0, z),
                               Landscape::grad(P[bb], x-1.0, y-1.0, z))),
                     Landscape::lerp(v, Landscape::lerp(u, Landscape::grad(P[aa+1], x    , y    , z-1.0),
                                  Landscape::grad(P[ba+1], x-1.0, y    , z-1.0)),
                          Landscape::lerp(u, Landscape::grad(P[ab+1], x    , y-1.0, z-1.0),
                               Landscape::grad(P[bb+1], x-1.0, y-1.0, z-1.0))));

         if height <= SEA as f64 { height = SEA as f64 }
         return height
     }

     fn fade(t: f64) -> f64 {
         t * t * t * ( t * (t * 6.0 - 15.0) + 10.0)
     }

     fn lerp(t: f64, a: f64, b: f64) -> f64 {
         a + t * (b - a)
     }

     fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
         let h = hash & 15;
         let u = if h < 8 { x } else { y };
         let v = if h < 4 { y } else { if h == 12 || h == 14 { x } else { z } };

         return if h&1 == 0 { u } else { -u } + if h&2 == 0 { v } else { -v };
     }

 }
     static P: [usize; 512] = [151,160,137,91,90,15,131,13,201,95,96,53,194,233,
         7,225,140,36,103,30,69,142,8,99,37,240,21,10,23,190, 6,148,247,120,234,
         75,0,26,197,62,94,252,219,203,117,35,11,32,57,177,33,88,237,149,56,87,
         174,20,125,136,171,168, 68,175,74,165,71,134,139,48,27,166,77,146,158,
         231,83,111,229,122,60,211,133,230,220,105,92,41,55,46,245,40,244,102,
         143,54, 65,25,63,161, 1,216,80,73,209,76,132,187,208, 89,18,169,200,196,
         135,130,116,188,159,86,164,100,109,198,173,186, 3,64,52,217,226,250,124,
         123,5,202,38,147,118,126,255,82,85,212,207,206,59,227,47,16,58,17,182,189,
         28,42,223,183,170,213,119,248,152, 2,44,154,163, 70,221,153,101,155,167,
         43,172,9,129,22,39,253, 19,98,108,110,79,113,224,232,178,185, 112,104,218,
         246,97,228,251,34,242,193,238,210,144,12,191,179,162,241, 81,51,145,235,249,
         14,239,107,49,192,214, 31,181,199,106,157,184, 84,204,176,115,121,50,45,127,
         4,150,254,138,236,205,93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,
         180,
         151,160,137,91,90,15,131,13,201,95,96,53,194,233,7,225,140,36,103,30,69
         ,142,8,99,37,240,21,10,23,190, 6,148,247,120,234,75,0,26,197,62,94,252,219,
         203,117,35,11,32,57,177,33,88,237,149,56,87,174,20,125,136,171,168, 68,175,
         74,165,71,134,139,48,27,166, 77,146,158,231,83,111,229,122,60,211,133,230,
         220,105,92,41,55,46,245,40,244,102,143,54, 65,25,63,161, 1,216,80,73,209,
         76,132,187,208, 89,18,169,200,196,135,130,116,188,159,86,164,100,109,198,
         173,186, 3,64,52,217,226,250,124,123,5,202,38,147,118,126,255,82,85,212,
         207,206,59,227,47,16,58,17,182,189,28,42,223,183,170,213,119,248,152, 2,
         44,154,163, 70,221,153,101,155,167, 43,172,9,129,22,39,253, 19,98,108,
         110,79,113,224,232,178,185, 112,104,218,246,97,228,251,34,242,193,238,
         210,144,12,191,179,162,241, 81,51,145,235,249,14,239,107, 49,192,214, 31,
         181,199,106,157,184, 84,204,176,115,121,50,45,127, 4,150,254,138,236,205,
         93,222,114,67,29,24,72,243,141,128,195,78,66,215,61,156,180];


