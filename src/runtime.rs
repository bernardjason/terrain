//use std::ffi::CString;
use std::time::Instant;

use cgmath::{Deg, Matrix4,  Point3, vec3, Angle};
use emscripten_main_loop::MainLoopEvent;
use sdl2::{Sdl, };
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::{GLContext, Window};

use crate::{get_start_time, gl, HEIGHT, WIDTH};
use crate::flying_camera::Flying_Camera;
use crate::gl_helper::draw_text::DrawText;
use crate::gl_helper::skybox::Skybox;
use crate::ground_map::GroundMap;
#[cfg(target_os = "emscripten")]
use crate::handle_javascript::end_game;
#[cfg(target_os = "emscripten")]
use crate::handle_javascript::start_game;
use crate::plane::Plane;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::openglshadow::OpenglShadow;
use crate::pickup::Pickup;
use rand::Rng;
use crate::special_effects::SpecialEffects;

//#[cfg(target_os = "emscripten")]


const TARGET_FPS: u128 = 30;
const PLANE_X:f32=-24.0;
const PLANE_Y:f32=4.0;
const PLANE_Z:f32=48.0;

enum SCREEN {
    START,
    MAIN,
    END

}
pub struct Runtime {
    screen: SCREEN,
    now: Instant,
    last_time_called: u128,
    rate_debug: String,
    sdl: Sdl,
    window: Window,
    pub gl: std::rc::Rc<gl::Gl>,
    _gl_context_keep: GLContext,
    pub camera: Flying_Camera,
    tick: u128,
    draw_text: Option<DrawText>,
    javascript_start_game_called:bool,
    pub one:bool,
    pub two:bool,
    pub three:bool,
    pub four:bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub forward: bool,
    pub backward: bool,
    pub space: bool,
    space_spacer:i128,
    pub ground_map: GroundMap,
    //pub no_shadow_shader: u32,
    opengl_shadow: OpenglShadow,
    sky_box:Skybox,
    plane:Plane,
    player:Player,
    bullets:Bullet,
    pickups:Pickup,
    special_effects:SpecialEffects,
    score:i32,
    shields:f32,
}



impl Runtime {
    pub(crate) fn new() -> Runtime {
        let start = get_start_time();
        let sdl = sdl2::init().unwrap();

        let video = sdl.video().unwrap();

        #[cfg(not(target_os = "emscripten"))]
            let context_params = (sdl2::video::GLProfile::Core, 3, 0);
        #[cfg(target_os = "emscripten")]
            let context_params = (sdl2::video::GLProfile::GLES, 3, 0);


        video.gl_attr().set_context_profile(context_params.0);
        video.gl_attr().set_context_major_version(context_params.1);
        video.gl_attr().set_context_minor_version(context_params.2);

        // Create a window
        let window = video
            .window("terrain", WIDTH, HEIGHT)
            .resizable()
            .opengl()
            .position_centered()
            .build().unwrap();


        let gl_context = window.gl_create_context().unwrap();

        let gl_orig: std::rc::Rc<gl::Gl> = std::rc::Rc::new(gl::Gl::load_with(|s| { video.gl_get_proc_address(s) as *const _ }));

        let gl = std::rc::Rc::clone(&gl_orig);

        let camera = Flying_Camera {
            Position: Point3::new(0.0, 3.0, -0.0),
            ..Flying_Camera::default()
        };

        unsafe { gl.Enable(gl::BLEND); }

        let ground_map = GroundMap::new(&gl);

        let opengl_shadow = OpenglShadow::new(&gl);
        //let no_shadow_shader= create_shader(&gl, SKYBOX_VS, SKYBOX_FS, None);

        let plane =Plane::new(&gl);

        let runtime = Runtime {
            screen: SCREEN::START,
            now: Instant::now(),
            last_time_called: 0,
            sdl,
            window,
            _gl_context_keep: gl_context,
            gl: gl_orig,
            camera,
            rate_debug: "".to_string(),
            tick: 0,
            draw_text: None,
            javascript_start_game_called:false,
            one: false,
            two: false,
            three: false,
            four: false,
            left: false,
            right: false,
            up: false,
            down: false,
            forward: false,
            backward: false,
            space: false,
            space_spacer:0,
            //no_shadow_shader:no_shadow_shader,
            opengl_shadow,
            ground_map: ground_map,
            sky_box : Skybox::new(&gl, "resources/sky.png"),
            plane,
            player:Player::new(&gl,vec3(PLANE_X,PLANE_Y,PLANE_Z)),
            bullets:Bullet::new(&gl),
            pickups:Pickup::new(&gl),
            special_effects:SpecialEffects::new(&gl),
            score:0,
            shields:100.0,

        };
        runtime
    }
}


impl emscripten_main_loop::MainLoop for Runtime {
    fn main_loop(&mut self) -> emscripten_main_loop::MainLoopEvent {
        self.tick = self.tick + 1;

        let debug_start = Instant::now();

        let time_now = self.now.elapsed().as_millis();
        let diff = time_now - self.last_time_called;

        if diff < 1000 / TARGET_FPS {
            return MainLoopEvent::Continue;
        }

        self.last_time_called = time_now;

        let delta = (diff as f32) / 1000.0;

        let fps = 1.0 / delta as f32;

        // just for browser, big drop in rate on first load
        //if fps > 5.0 { 1.0 } else { fps };

        if self.tick % 20 <= 1 {
            self.rate_debug = format!("{:2.2}", fps);
        }


        unsafe {
            self.gl.Enable(gl::DEPTH_TEST);
        }

        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
            self.gl.Clear(gl::DEPTH_BUFFER_BIT | gl::COLOR_BUFFER_BIT);
        }

        if ! self.javascript_start_game_called && self.draw_text.is_some() {
            self.javascript_start_game_called = true;
            #[cfg(target_os = "emscripten")]
                unsafe {
                start_game();
            }
        }
        if self.tick > 2 && self.draw_text.is_none() {
            self.setup_text_if_not_loaded();
        }

        if self.tick > 3 {
            match self.screen {
                SCREEN::START => self.start_screen(delta),
                SCREEN::MAIN => self.game_screen(delta),
                SCREEN::END => self.end_screen(delta),
            }
        }

        self.window.gl_swap_window();

        let end_status = self.handle_keyboard();

        match end_status {
            MainLoopEvent::Terminate => {
                #[cfg(target_os = "emscripten")]
                    unsafe {
                    end_game();
                }
            }
            MainLoopEvent::Continue => {}
        }

        end_status
    }
}


impl Runtime {
    fn setup_text_if_not_loaded(&mut self) {
        let start_block = Instant::now();
        let draw_text = DrawText::new(&self.gl);
        let duration = start_block.elapsed();
        println!("Time elapsed in drawtext is: {:?}", duration);
        self.draw_text = Some(draw_text);
    }


    fn handle_keyboard(&mut self) -> MainLoopEvent {
        let mut return_status = emscripten_main_loop::MainLoopEvent::Continue;
        let mut events = self.sdl.event_pump().unwrap();

        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return_status = emscripten_main_loop::MainLoopEvent::Terminate;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    self.left = true;
                    self.right = false;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    self.right = true;
                    self.left = false;
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    self.up = true;
                    self.down = false
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    self.down = true;
                    self.up = false
                }
                Event::KeyDown { keycode: Some(Keycode::LShift), .. } => {
                    self.forward = true;
                }
                Event::KeyDown { keycode: Some(Keycode::RShift), .. } => {
                    self.backward = true;
                }
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    self.space_spacer = self.space_spacer+1;
                    if self.space_spacer >= 0 {
                        self.space = true;
                        self.space_spacer = -12;
                    } else {
                        self.space = false;
                    }
                }
                Event::KeyUp { keycode: Some(Keycode::Left), .. } => { self.left = false; }
                Event::KeyUp { keycode: Some(Keycode::Right), .. } => { self.right = false; }
                Event::KeyUp { keycode: Some(Keycode::Up), .. } => { self.up = false }
                Event::KeyUp { keycode: Some(Keycode::Down), .. } => { self.down = false }
                Event::KeyUp { keycode: Some(Keycode::LShift), .. } => { self.forward = false }
                Event::KeyUp { keycode: Some(Keycode::RShift), .. } => { self.backward = false }
                Event::KeyUp { keycode: Some(Keycode::Space), .. } => {
                    self.space = false;
                    self.space_spacer = 0;
                }

                Event::KeyUp { keycode: Some(Keycode::Num1), .. } => { self.one = false; }
                Event::KeyDown { keycode: Some(Keycode::Num1), .. } => { self.one = true; }
                Event::KeyUp { keycode: Some(Keycode::Num2), .. } => { self.two = false; }
                Event::KeyDown { keycode: Some(Keycode::Num2), .. } => { self.two = true; }
                Event::KeyUp { keycode: Some(Keycode::Num3), .. } => { self.three = false; }
                Event::KeyDown { keycode: Some(Keycode::Num3), .. } => { self.three = true; }
                Event::KeyUp { keycode: Some(Keycode::Num4), .. } => { self.four = false; }
                Event::KeyDown { keycode: Some(Keycode::Num4), .. } => { self.four = true; }
                _ => {}
            }
        }

        return_status
    }
}

const ALWAYS_WANT_THIS_NUMBER_OF_PLANES: usize = 5;


impl Runtime {
    fn game_screen(&mut self,delta:f32) {
        self.ground_map.update(
            &self.gl, self.camera.Position.x, self.camera.Position.z, self.player.rotation_around_ship(),
            &mut self.pickups,
        );

        self.special_effects.update(delta);
        self.score = self.score + self.pickups.update_pickups(delta,&self.player.position);

        self.shields=self.shields+0.3 * delta;
        if self.shields >= 100.0 {
            self.shields=100.0;
        }
        if self.shields <= 0.0 {
            self.screen = SCREEN::END;
        }
        if self.one {
            self.shields=-1.0;
        }

        let (view, projection) = self.player_movement(delta);

        self.plane.update(self.player.position, delta, &mut self.bullets, &self.ground_map, &mut self.special_effects);

        let (mut calc_sun_pos,player_dir)= self.player.get_bullet_direction();

        calc_sun_pos = calc_sun_pos - player_dir*8.0; // trade off about shadow on ship but i want shadow on ground.
        let mut deg = Deg::atan2(player_dir.x, player_dir.z).0 + 180.0;

        if deg < 0.0 { deg = deg + 360.0; }
        self.opengl_shadow.update_light_pos(
            calc_sun_pos.x, 6.0, calc_sun_pos.z,
            deg);


        for i in self.bullets.instances.iter_mut() {
            if i.enemy == false && self.plane.been_hit(i.position, &mut self.special_effects) {
                i.mark_finished();
                self.score=self.score+10;
            }
            if i.enemy == true && self.player.been_hit(i.position) {
                i.mark_finished();
                self.special_effects.explosion(self.player.position);
                self.shields=self.shields-10.0;
            }
        }
        if self.plane.been_hit(self.player.position, &mut self.special_effects) {
            self.special_effects.explosion(self.player.position);
            self.player.rollback();
            self.shields=self.shields-10.0;
        }

        if self.plane.instances.len() < ALWAYS_WANT_THIS_NUMBER_OF_PLANES {
            let mut rng = rand::thread_rng();

            let (mut pos, mut dir) = self.player.get_bullet_direction();
            dir=dir  * -1.0;
            pos = pos - dir * 5.0;
            pos.x = pos.x + ( rng.gen_range(-7,7) as f32 );
            pos.z = pos.z + ( rng.gen_range(-7,7) as f32 );

            pos.y=4.0;
            self.plane.new_instance(&self.gl, pos,  );



        }


        self.opengl_shadow.start_render_shadow(&self.gl);
        self.plane.render(&self.gl ,&view,&projection,self.opengl_shadow.simple_depth_shader,true);
        self.player.render(&self.gl,&view,&projection,self.opengl_shadow.simple_depth_shader);

        self.opengl_shadow.after_rendersceneshadow(&self.gl);


        self.opengl_shadow.before_renderscenenormal(&self.gl, vec3(self.camera.Position.x, self.camera.Position.y, self.camera.Position.z));
        self.ground_map.render(&self.gl ,&view,&projection,self.opengl_shadow.shader);
        self.plane.render(&self.gl ,&view,&projection,self.opengl_shadow.shader,false);
        self.player.render(&self.gl,&view,&projection,self.opengl_shadow.shader);

        //self.sky_box.render(&self.gl, &view, &projection, self.camera.Position.to_vec());


        self.bullets.update_bullets(delta);
        self.bullets.render(&self.gl,&view,&projection);
        self.pickups.render(&self.gl,&view,&projection,self.opengl_shadow.shader);
        self.special_effects.render(&self.gl,&view,&projection,self.opengl_shadow.shader);

        self.display_score();
    }

    fn display_score(&mut self) {
        let status = format!("fps={} shields={:2.0} hit={} score={} ",
                             self.rate_debug,
                             if self.shields >= 0.0 {
                                 self.shields
                             } else {
                                 0.0
                             },
                             self.player.been_hit, self.score);
        self.draw_text.as_ref().unwrap().draw_text(&self.gl, &status, 2.0, HEIGHT as f32 - 30.0, vec3(1.0, 1.0, 0.0), 1.0);
    }

    fn player_movement(&mut self, delta: f32) -> (Matrix4<f32>, Matrix4<f32>) {
        let (view, projection) = self.player.projection_view_camera(&mut self.camera);
        self.player.debug_positiom(self.one,self.two,self.three,self.four);
        self.player.update_player();

        if self.space {
            let (pos,  dir) = self.player.get_bullet_direction();
            self.bullets.new_instance(dir,pos,vec3(0.8,0.0,1.0),false);
        }

        if self.left { self.player.process_keyboard_roll(-80.0, delta); }
        if self.right { self.player.process_keyboard_roll(80.0, delta); }
        if self.up { self.player.process_keyboard_pitch(80.0, delta); }
        if self.down { self.player.process_keyboard_pitch(-80.0, delta); }
        if self.forward {
            self.player.forward_pressed(0.8);
        }
        self.player.forward(1.0, delta);
        self.player.slow_down_resistance();


        if ! self.player.player_reset && self.ground_map.height(self.player.position.x, self.player.position.z) >= self.player.position.y - 0.1 {
            self.player.rollback();
            self.special_effects.explosion(self.player.position);
            self.special_effects.explosion(self.player.position + vec3(0.0,0.1,0.0));
            self.shields=self.shields-10.0;
        }

        if self.up == false && self.down == false {
            self.player.update_camera();
        } else {
            self.player.queue_camera_for_later();
        }
        (view, projection)
    }
    fn start_screen(&mut self,delta:f32) {
        self.background(delta);

        let status = format!("Press space to start");
        self.draw_text.as_ref().expect("draw text not set").draw_text(&self.gl, &status, 16.0, HEIGHT as f32 - 230.0,
                                                                      vec3(1.0, 1.0, 0.0), 2.0);

        let status = format!("Arrow keys to fly\nSpace to fire\nLeft shift to speed up\nYou can practice now");
        self.draw_text.as_ref().expect("draw text not set").draw_text(&self.gl, &status, 16.0, HEIGHT as f32 /2.0,
                                                                      vec3(1.0, 1.0, 0.0), 1.0);
        if self.space {
            self.score=0;
            self.shields=100.0;
            self.player.position = vec3(PLANE_X,PLANE_Y,PLANE_Z);
            self.player.reset();
            self.player.been_hit=0;
            self.screen = SCREEN::MAIN;
            self.space=false;
        }

    }

    fn background(&mut self, delta: f32) {
        self.ground_map.update(
            &self.gl, self.camera.Position.x, self.camera.Position.z, self.player.rotation_around_ship(),
            &mut self.pickups,
        );

        let (view, projection) = self.player_movement(delta);

        self.opengl_shadow.before_renderscenenormal(&self.gl, vec3(self.camera.Position.x, self.camera.Position.y, self.camera.Position.z));
        self.ground_map.render(&self.gl, &view, &projection, self.opengl_shadow.shader);
        self.player.render(&self.gl, &view, &projection, self.opengl_shadow.shader);
    }
    fn end_screen(&mut self,delta:f32) {
        self.background(delta);
        self.display_score();
        let status = format!("Game over");
        self.draw_text.as_ref().unwrap().draw_text(&self.gl, &status, 32.0, HEIGHT as f32/2.0, vec3(1.0, 1.0, 0.0), 2.0);
        if self.space {
            self.space=false;
            self.screen = SCREEN::START;
        }
    }
}
