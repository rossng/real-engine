// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate glutin;
extern crate gfx_window_glutin;

use cgmath::{Deg, Matrix4, Point3, Vector3};
use gfx::{ texture, traits::FactoryExt, Device, Factory};
use glutin::GlContext;

mod scene;
pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

// Declare the vertex format suitable for drawing,
// as well as the constants used by the shaders
// and the pipeline state object format.
// Notice the use of FixedPoint.
gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    constant Locals {
        transform: [[f32; 4]; 4] = "u_Transform",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        transform: gfx::Global<[[f32; 4]; 4]> = "u_Transform",
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",
        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}


impl Vertex {
    fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
        Vertex {
            pos: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
            tex_coord: [t[0] as f32, t[1] as f32],
        }
    }
}


fn default_view() -> Matrix4<f32> {
    Matrix4::look_at(
        Point3::new(1.5f32, -5.0, 3.0),
        Point3::new(0f32, 0.0, 0.0),
        Vector3::unit_z(),
    )
}

pub fn main() {
   let cube_verts = [
    // top (0, 0, 1)
    Vertex::new([-1, -1,  1], [0, 0]),
    Vertex::new([ 1, -1,  1], [1, 0]),
    Vertex::new([ 1,  1,  1], [1, 1]),
    Vertex::new([-1,  1,  1], [0, 1]),
    // bottom (0, 0, -1)
    Vertex::new([-1,  1, -1], [1, 0]),
    Vertex::new([ 1,  1, -1], [0, 0]),
    Vertex::new([ 1, -1, -1], [0, 1]),
    Vertex::new([-1, -1, -1], [1, 1]),
    // right (1, 0, 0)
    Vertex::new([ 1, -1, -1], [0, 0]),
    Vertex::new([ 1,  1, -1], [1, 0]),
    Vertex::new([ 1,  1,  1], [1, 1]),
    Vertex::new([ 1, -1,  1], [0, 1]),
    // left (-1, 0, 0)
    Vertex::new([-1, -1,  1], [1, 0]),
    Vertex::new([-1,  1,  1], [0, 0]),
    Vertex::new([-1,  1, -1], [0, 1]),
    Vertex::new([-1, -1, -1], [1, 1]),
    // front (0, 1, 0)
    Vertex::new([ 1,  1, -1], [1, 0]),
    Vertex::new([-1,  1, -1], [0, 0]),
    Vertex::new([-1,  1,  1], [0, 1]),
    Vertex::new([ 1,  1,  1], [1, 1]),
    // back (0, -1, 0)
    Vertex::new([ 1, -1,  1], [0, 0]),
    Vertex::new([-1, -1,  1], [1, 0]),
    Vertex::new([-1, -1, -1], [1, 1]),
    Vertex::new([ 1, -1, -1], [0, 1]),
    ];

    let cube_idx : &[u16] = &[
        0,  1,  2,  2,  3,  0, // top
        4,  5,  6,  6,  7,  4, // bottom
        8,  9, 10, 10, 11,  8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];

    let mut events_loop = glutin::EventsLoop::new();
    let window_config = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        .with_dimensions(1024, 768);
    
        let vs = gfx_app::shade::Source {
            glsl_120: include_bytes!("../shader/cube_120.glslv"),
            glsl_150: include_bytes!("../shader/cube_150_core.glslv"),
            glsl_es_100: include_bytes!("../shader/cube_100_es.glslv"),
            glsl_es_300: include_bytes!("../shader/cube_300_es.glslv"),
            hlsl_40:  include_bytes!("../data/vertex.fx"),
            msl_11: include_bytes!("../shader/cube_vertex.metal"),
            vulkan:   include_bytes!("../data/vert.spv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_120: include_bytes!("../shader/cube_120.glslf"),
            glsl_150: include_bytes!("../shader/cube_150_core.glslf"),
            glsl_es_100: include_bytes!("../shader/cube_100_es.glslf"),
            glsl_es_300: include_bytes!("../shader/cube_300_es.glslf"),
            hlsl_40:  include_bytes!("../data/pixel.fx"),
            msl_11: include_bytes!("../shader/cube_frag.metal"),
            vulkan:   include_bytes!("../data/frag.spv"),
            .. gfx_app::shade::Source::empty()
        };
    let (api, version) = if cfg!(target_os = "emscripten") {
        (
            glutin::Api::WebGl, (2, 0),
        )
    } else {
        (
            glutin::Api::OpenGl, (3, 2),
        )
    };

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(api, version))
        .with_vsync(true);
    let (window, mut device, mut factory, main_color, mut main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(window_config, context, &events_loop);
    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());

    let pso = factory.create_pipeline_simple(
            &vs.glsl_120.to_vec(),
            &ps.glsl_120.to_vec(),
            pipe::new()
    ).unwrap();
    
    let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&cube_verts, cube_idx);
    let texels = [[0x20, 0xA0, 0xC0, 0xFF]];
    let (_, texture_view) = factory.create_texture_immutable::<gfx::format::Rgba8>(
        texture::Kind::D2(1, 1, texture::AaMode::Single), texture::Mipmap::Provided, &[&texels]
    ).unwrap();

    let sinfo = texture::SamplerInfo::new(
        texture::FilterMethod::Bilinear,
        texture::WrapMode::Clamp);

    let proj = cgmath::perspective(Deg(45.0f32), 1.3333, 1.0, 10.0);

    let mut data = pipe::Data {
        vbuf: vbuf,
        transform: (proj * default_view()).into(),
        locals: factory.create_constant_buffer(1),
        color: (texture_view, factory.create_sampler(sinfo)),
        out_color: main_color,
        out_depth: main_depth,
    };

    let mut scene = scene::Scene::default();
    events_loop.run_forever(move |event| {
        use glutin::{ControlFlow, Event, KeyboardInput, VirtualKeyCode, WindowEvent};

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested |
                WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                    ..
                } => return ControlFlow::Break,
                WindowEvent::Resized(width, height) => {
                    window.resize(width, height);
                    gfx_window_glutin::update_views(&window, &mut data.out_color, &mut data.out_depth);
                },
                WindowEvent::KeyboardInput{device_id, input}  => println!("Got key! {}", input.scancode),
                _ => (),
            }
        }

        // draw a frame
        let locals = Locals { transform: data.transform };
        encoder.update_constant_buffer(&data.locals, &locals);
        scene.update();
        /* Update projection with camera from scene*/
        data.transform = (proj *scene.camera.transform()).into();
        encoder.clear(&data.out_color, [0.1, 0.2, 0.3, 1.0]);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();

        ControlFlow::Continue
    });
}