/*
    Handles the setup of our Pipeline State Object (PSO)
*/
pub mod pso {
    use gfx;
    /*
    Define the input data formats and pipeline.
    */
    pub type ColorFormat = gfx::format::Rgba8;
    pub type DepthFormat = gfx::format::DepthStencil;

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
    /*
    Traits for the host-side vertex object
    */
    impl Vertex {
        pub fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
            Vertex {
                pos: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
                tex_coord: [t[0] as f32, t[1] as f32],
            }
        }
    }
}