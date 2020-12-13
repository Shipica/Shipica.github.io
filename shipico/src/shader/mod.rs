pub mod post_processing_shader {
    use miniquad::*;

    pub const VERTEX: &str = include_str!("post_process.vert");
    pub const FRAGMENT: &str = include_str!("post_process.frag");

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("resolution", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub resolution: glam::Vec2,
    }
}

pub mod offscreen_shader {
    use miniquad::*;

    pub const VERTEX: &str = include_str!("default.vert");
    pub const FRAGMENT: &str = include_str!("default.frag");

    // @NOTE: Meta information must contain information about the shader uniforms for
    // framework to correctly pass data on the draw call.
    // So, if you will change the shader, you must change meta information.
    //
    // Currently, there are only one uniform variable and it is the view matrix of the camera
    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("u_matrix", UniformType::Mat3)],
                // @NOTE              Be sure to match uniform type here ^^^^ with type
                // of the data in `Uniform` struct below
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat3,
    }
}
