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

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub mvp: glam::Mat4,
    }
}
