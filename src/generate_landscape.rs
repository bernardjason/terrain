use crate::ground_cell::{SEA, PAST_SAND};

/*
thanks to https://gist.github.com/Fataho/5b422037a6fdcb21c9134ef34d2fa79a
 */
pub struct GenerateLandscape {
    //perlin:Perlin
}
 impl GenerateLandscape {
     pub fn new() -> GenerateLandscape {

         //let perlin = Perlin::new(2);

         GenerateLandscape {
             //perlin
         }
     }

     pub fn test_it(&mut self) {

         let val = GenerateLandscape::noise(3.14 , 7.0 )  ;
         println!("BASE {}",val);

         let mut max = 0.0;
         let mut min = 0.0;
         for xx in (-10..11).step_by(1) {
             for zz in (-10..11).step_by(1) {
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

     pub fn pseudo_random(&self, x:f64, z:f64 ) -> f64 {

         let frequency = 0.0080;
         let time = 0.01;


         let mut val = GenerateLandscape::noise((x * frequency)+ time , (z * frequency) + time) ;

         if val <= SEA as f64{
             val = SEA as f64;
         } else if val >= PAST_SAND as f64  {
             val = val * 10.0;
         }

         val
     }


     fn noise(x: f64, y: f64) -> f64 {
        let xi = x.floor() as usize & 255;
         let yi = y.floor() as usize & 255;
         let g1 = P[P[xi] + yi];
         let g2 = P[P[xi +1] + yi];
         let g3 = P[P[xi] + yi +1];
         let g4 = P[P[xi +1] + yi+1];

         let xf = x - x.floor();
         let yf = y - y.floor();

         let d1 = GenerateLandscape::grad(g1,xf,yf);
         let d2 = GenerateLandscape::grad(g2,xf-1.0,yf);
         let d3 = GenerateLandscape::grad(g3,xf,yf-1.0);
         let d4 = GenerateLandscape::grad(g4,xf-1.0,yf-1.0);

         let u = GenerateLandscape::fade(xf);
         let v = GenerateLandscape::fade(yf);

         let x1inter = GenerateLandscape::lerp(u, d1, d2);
         let x2inter = GenerateLandscape::lerp(u, d3, d4);
         let y1inter = GenerateLandscape::lerp(v, x1inter, x2inter);


         return y1inter;
     }


     fn grad(hash: usize, x: f64, y: f64) -> f64 {
         let h = hash & 3;
         return match h {
             0 => x + y,
             1 => -x + y,
             2 => x - y,
             3 => -x - y,
             _ => 0.0,
         }
     }


     fn lerp(amount: f64, left: f64, right: f64) -> f64 {
         (1.0 - amount) * left + amount * right
     }


     fn fade(t: f64) -> f64 { t * t * t * ( t * (t * 6.0 - 15.0) + 10.0) }
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


