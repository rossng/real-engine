extern crate cgmath;
extern crate glutin;
use cgmath::{Deg, Matrix4, Point3, Vector3};

#[derive(Copy, Clone)]
pub struct Camera {
    pub loc: Point3<f32>,
    pub view: Point3<f32>,
}

impl Camera {
    pub fn transform(&self) -> Matrix4<f32> {
         Matrix4::look_at(
            self.loc,
            self.view,
            Vector3::unit_z(),
        )
    }

    pub fn default() -> Camera {
        Camera{loc: Point3::new(1.5f32, -5.0, 3.0), view: Point3::new(0f32, 0.0, 0.0)}
    }
}

pub struct PlayerInput {
    pub events_loop : glutin::EventsLoop,
}

impl PlayerInput {
    pub fn default() -> PlayerInput {
        PlayerInput {events_loop: glutin::EventsLoop::new()}
    }

    pub fn get_key(&mut self) -> () {
        self.events_loop.poll_events(|event| {
                    match event {
                        glutin::Event::WindowEvent{ event, .. } => match event {
                            glutin::WindowEvent::KeyboardInput{device_id, input}  => println!("Got key! {}", input.scancode),
                            _ => ()
                        },
                        _ => ()
                    }
            });
    }
}

pub struct Scene {
    pub frame: i32,
    pub camera: Camera,
    pub player_input: PlayerInput,
}


impl Scene {
    pub fn update(&mut self) -> () {
        self.frame += 1;
        self.camera.loc.x += 0.01;
        self.player_input.get_key();
    }

    pub fn default() -> Scene {
        Scene{frame: 1, camera: Camera::default(), player_input: PlayerInput::default() }
    }
    //pub fn get_camera(&self) -> Camera {
     //   self.camera
    //}
}