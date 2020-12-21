#![feature(unsized_fn_params)]

use glam::{vec2, vec4, Mat3, Vec2, Vec4};
use miniquad::*;

mod shader;
use shader::*;

struct Input {
    mouse_down: bool,
    last_mouse_pos: Vec2,
}

struct Camera {
    position: Vec2,
    zoom: f32,
}

struct Stage {
    node_pipeline: Pipeline,
    workbench_pipeline: Pipeline,
    node: Node,
    input: Input,
    camera: Camera,
    workbench: Workbench,
}

const PERFECT_SIZE: (f32, f32) = (1000., 1000.);

impl Stage {
    // @NOTE:
    // This function will be executed once at the start of the program.
    // Right after the `main` is called.
    pub fn new(ctx: &mut Context) -> Stage {
        let node_shader = Shader::new(
            ctx,
            offscreen_shader::VERTEX,
            offscreen_shader::FRAGMENT,
            offscreen_shader::meta(),
        )
        .unwrap();

        let node_pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("a_position", VertexFormat::Float2)],
            node_shader,
            PipelineParams {
                primitive_type: PrimitiveType::Triangles,
                cull_face: CullFace::Nothing,
                ..Default::default()
            },
        );

        let workbench_shader = Shader::new(
            ctx,
            workbench_shader::VERTEX,
            workbench_shader::FRAGMENT,
            workbench_shader::meta(),
        )
        .unwrap();

        let workbench_pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("a_position", VertexFormat::Float2)],
            workbench_shader,
            PipelineParams {
                primitive_type: PrimitiveType::Triangles,
                cull_face: CullFace::Nothing,
                ..Default::default()
            },
        );

        Stage {
            node_pipeline,
            workbench_pipeline,
            node: Node::new(ctx),
            workbench: Workbench::new(ctx),
            input: Input {
                mouse_down: false,
                last_mouse_pos: Vec2::zero(),
            },
            camera: Camera {
                position: Vec2::zero(),
                zoom: 1.0,
            },
        }
    }
}

/// Transforms screen space point to local space point.
///
/// Screen space starts in top left corner of the screen with (0., 0.) coordinates
/// and ends in bottom right corner with (screen_width, screen_height) coordinates.
///
/// Local space starts in center of the screen with (0., 0.) coordinates
/// with (-1, 0) in the left corner of the screen, (1, 0) in the right corner,
/// (0, 1) at the top and (0, -1) at the bottom of the screen.
///
/// ### Examples:
/// `(screen_width / 2, screen_height / 2)` -> `(0, 0)`.
///
/// `(0, 0)` -> `(-1, 1)`.
///
/// `(screen_width, 0)` -> `(1, 1)`.
fn screen_to_local(mut mouse_pos: Vec2, screen_size: Vec2) -> Vec2 {
    mouse_pos -= screen_size / 2.0;

    // y is faced down. We need it to face up.
    mouse_pos = vec2(mouse_pos.x, -mouse_pos.y);

    // -0.5..0.5
    mouse_pos /= screen_size;

    // -1..1
    mouse_pos *= 2.0;

    mouse_pos
}

impl EventHandler for Stage {
    fn update(&mut self, ctx: &mut Context) {
        // it is called every frame in case you would want to change something with time or so.
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.input.mouse_down = true;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.input.mouse_down = false;
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let mouse_pos = vec2(x, y);
        // I don't exactly remember why this piece of code works, but it does,
        // so i do not recommend to touch it.
        //
        // Oh, it drags the camera view with the mouse.
        if self.input.mouse_down {
            let screen_size = ctx.screen_size();
            let screen_size = vec2(screen_size.0, -screen_size.1);
            let mut delta = self.input.last_mouse_pos - mouse_pos;
            delta = delta * 2.0 / screen_size;
            self.camera.position -= delta;
        }

        // Be sure to save mouse position every time it moves.
        self.input.last_mouse_pos = mouse_pos;
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        // On mouse wheel we zoom in and out.
        //
        // Wheel delta values are different in different browsers,
        // so we use constant values here to provide consistency.
        let zoom = if y > 0. { 1.05 } else { 0.95 };

        // This thing scales around center of the screen.
        //
        // The goal is to scale around mouse position.
        // Current mouse position may be obtained by using `self.input.last_mouse_pos`.
        // Current mouse position in in screen space, e.g. (0..1920).
        // To translate it to the local space (-1..1) you need to call `screen_to_local` function.
        // Screen sizes may be obtained by calling `ctx.screen_size()`.
        //
        // I don't know how to make it work.
        //
        // If it's nessessary you can replace `self.transform`
        // with `self.position` and `self.zoom` (in data structure and the rest of the code)
        // and work with them.
        //
        // TODO: Make scale work
        self.camera.zoom *= zoom;
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        // Resize now works from top left corner.
        //
        // It means that objects that were stuck at left border of the screen will
        // retain their `x` position after resize.
        //
        // Maybe it's better to resize objects keeping center position
    }

    fn draw(&mut self, ctx: &mut Context) {
        let (w, h) = ctx.screen_size();
        let w = w / PERFECT_SIZE.0;
        let h = h / PERFECT_SIZE.1;
        let mvp = m3::projection(w, h)
            * m3::translation(self.camera.position)
            * m3::scaling(self.camera.zoom);

        // Clear color buffer with white color
        ctx.begin_default_pass(PassAction::Clear {
            color: Some((1., 1., 1., 1.)),
            depth: None,
            stencil: None,
        });
        // Prepare shaders (gl.useProgram), set face culling, depth tests and such shit
        ctx.apply_pipeline(&self.workbench_pipeline);
        self.workbench.draw(&self.camera, ctx);

        ctx.apply_pipeline(&self.node_pipeline);
        self.node.draw(mvp, ctx);

        // Do some framework related job
        // It's nessesary to do after each pass.
        ctx.end_render_pass();

        // Do some framework related job
        // It must be the last framework related command in the draw function.
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}

// https://webglfundamentals.org/webgl/lessons/webgl-matrix-vs-math.html
/// Set of easy to understand matrix functions.
mod m3 {
    use glam::{Mat3, Vec2};

    #[rustfmt::skip]
    pub fn translation(d: Vec2) -> Mat3 {
        Mat3::from_cols_array(&[
             1.,  0.,  0.,
             0.,  1.,  0.,
            d.x, d.y,  1.,
        ])
    }

    #[rustfmt::skip]
    pub fn scaling(s: f32) -> Mat3 {
        Mat3::from_cols_array(&[
             s, 0.,  0.,
            0.,  s,  0.,
            0., 0.,  1.,
        ])
    }

    #[rustfmt::skip]
    pub fn rotation(angle: f32) -> Mat3 {
        let cos = angle.cos();
        let sin = angle.sin();
        Mat3::from_cols_array(&[
            cos,-sin,  0.,
            sin, cos,  0.,
             0.,  0.,  1.,
        ])
    }

    #[rustfmt::skip]
    pub fn identity() -> Mat3 {
        Mat3::from_cols_array(&[
            1.,  0.,  0.,
            0.,  1.,  0.,
            0.,  0.,  1.,
        ])
    }

    /// In 2d there are only ortho projection available ofcourse
    #[rustfmt::skip]
    pub fn projection(width: f32, height: f32) -> Mat3 {
        let w = 2. / width;
        let h = 2. / height;
        Mat3::from_cols_array(&[
              w,  0.,  0.,
             0.,   h,  0.,
            -1.,  1.,  1.,
        ])
    }

    /*
        translate: function(m, tx, ty) {
      return m3.multiply(m, m3.translation(tx, ty));
    },

    rotate: function(m, angleInRadians) {
      return m3.multiply(m, m3.rotation(angleInRadians));
    },

    scale: function(m, sx, sy) {
      return m3.multiply(m, m3.scaling(sx, sy));
    },
      */
}

use lyon::{
    math::{rect, Point},
    tessellation::{
        basic_shapes::*, geometry_builder::simple_builder, FillOptions, StrokeOptions,
        VertexBuffers,
    },
};

struct Workbench {
    rect: Bindings,
    background_color: Vec4,
    line_color: Vec4,
}

impl Workbench {
    fn new(ctx: &mut Context) -> Workbench {
        let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();
        let size = 2.;
        fill_rectangle(
            &rect(size / -2., size / -2., size, size),
            &FillOptions::tolerance(0.1),
            &mut simple_builder(&mut geometry),
        )
        .unwrap();

        let rect = Bindings {
            vertex_buffers: vec![Buffer::immutable(
                ctx,
                BufferType::VertexBuffer,
                &geometry.vertices,
            )],
            index_buffer: Buffer::immutable(ctx, BufferType::IndexBuffer, &geometry.indices),
            images: vec![],
        };

        Workbench {
            rect,
            background_color: rgba_from_hex("#70798c"),
            line_color: rgba_from_hex("#fff"),
        }
    }

    fn draw(&self, camera: &Camera, ctx: &mut Context) {
        // Push vertices, indices and textures of the model to the shader
        ctx.apply_bindings(&self.rect);
        // Push transform matrix to the uniforms of the shader
        ctx.apply_uniforms(&workbench_shader::Uniforms {
            position: camera.position,
            zoom: camera.zoom,
            back_color: self.background_color,
            line_color: self.line_color,
            resolution: vec2(ctx.screen_size().0, ctx.screen_size().1),
        });
        // Draw 1 instance of the model containing 12 triangles (36 indices) of the first (0) model in the bindings
        ctx.draw(0, (self.rect.index_buffer.size() / 2) as i32, 1);
    }
}

struct Node {
    border: Bindings,
    border_color: Vec4,

    background: Bindings,
    background_color: Vec4,
}

impl Node {
    fn new(ctx: &mut Context) -> Node {
        // @Thought
        // Tolerance from zoom maybe? (try playing with value to understand what i mean)
        //
        // I though it would be good to tesselate all the meshes one time on the start
        // but if we will change tolerance with every wheel move we will have to regenerate
        // mesh data. It will be awful from memory point of view.
        //
        // Way better, IMHO, use some kind of LOD system (have to be written).
        // That way we will generate N meshes for every little thing at the start
        // and will swap them as zoom changes.
        let border_radii = BorderRadii::new_all_same(10.);
        let rect = rect(0.0, 0.0, 200.0, 100.0);

        let background = {
            let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();

            let options = FillOptions::tolerance(0.05);

            fill_rounded_rectangle(
                &rect,
                &border_radii,
                &options,
                &mut simple_builder(&mut geometry),
            )
            .unwrap();

            Bindings {
                vertex_buffers: vec![Buffer::immutable(
                    ctx,
                    BufferType::VertexBuffer,
                    &geometry.vertices,
                )],
                index_buffer: Buffer::immutable(ctx, BufferType::IndexBuffer, &geometry.indices),
                images: vec![],
            }
        };

        let border = {
            let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();

            let options = StrokeOptions::tolerance(0.05);

            stroke_rounded_rectangle(
                &rect,
                &border_radii,
                &options,
                &mut simple_builder(&mut geometry),
            )
            .unwrap();

            Bindings {
                vertex_buffers: vec![Buffer::immutable(
                    ctx,
                    BufferType::VertexBuffer,
                    &geometry.vertices,
                )],
                index_buffer: Buffer::immutable(ctx, BufferType::IndexBuffer, &geometry.indices),
                images: vec![],
            }
        };

        Node {
            border,
            border_color: rgba_from_hex("#f5f1ed"),
            background,
            background_color: rgba_from_hex("#25232388"),
        }
    }

    fn draw(&self, mvp: Mat3, ctx: &mut Context) {
        // Push vertices, indices and textures of the model to the shader
        ctx.apply_bindings(&self.background);
        // Push transform matrix to the uniforms of the shader
        ctx.apply_uniforms(&offscreen_shader::Uniforms {
            mvp,
            color: self.background_color,
        });
        // Draw 1 instance of the model containing 12 triangles (36 indices) of the first (0) model in the bindings
        ctx.draw(0, (self.background.index_buffer.size() / 2) as i32, 1);

        // Push vertices, indices and textures of the model to the shader
        ctx.apply_bindings(&self.border);
        // Push transform matrix to the uniforms of the shader
        ctx.apply_uniforms(&offscreen_shader::Uniforms {
            mvp,
            color: self.border_color,
        });
        // Draw 1 instance of the model containing 12 triangles (36 indices) of the first (0) model in the bindings
        ctx.draw(0, (self.border.index_buffer.size() / 2) as i32, 1);
    }
}

/// Color hex to vec4.
///
/// ### Examples:
///
/// `#fff` -> `vec4(1., 1., 1., 1.)`
///
/// `#C0C0C0` -> `vec4(1., 1., 1., 1.)`
///
/// `#ffffff00` -> `vec4(1., 1., 1., 0.)`
///
/// ### Panics:
/// If provided string is not valid color hex.
#[rustfmt::skip]
fn rgba_from_hex(hex: &str) -> Vec4 {
    let len = hex.len();
    assert!(&[4, 5, 7, 9].contains(&len));

    use std::u8;

    match len {
        4 => {
            let r = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).unwrap() as f32 / 255.;
            let g = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).unwrap() as f32 / 255.;
            let b = u8::from_str_radix(&format!("{}{}", &hex[3..4], &hex[3..4]), 16).unwrap() as f32 / 255.;
            vec4(r, g, b, 1.)
        }
        5 => {
            let r = u8::from_str_radix(&format!("{}{}", &hex[1..2], &hex[1..2]), 16).unwrap() as f32 / 255.;
            let g = u8::from_str_radix(&format!("{}{}", &hex[2..3], &hex[2..3]), 16).unwrap() as f32 / 255.;
            let b = u8::from_str_radix(&format!("{}{}", &hex[3..4], &hex[3..4]), 16).unwrap() as f32 / 255.;
            let a = u8::from_str_radix(&format!("{}{}", &hex[4..5], &hex[4..5]), 16).unwrap() as f32 / 255.;
            vec4(r, g, b, a)
        }
        7 => {
            let r = u8::from_str_radix(&hex[1..3], 16).unwrap() as f32 / 255.;
            let g = u8::from_str_radix(&hex[3..5], 16).unwrap() as f32 / 255.;
            let b = u8::from_str_radix(&hex[5..7], 16).unwrap() as f32 / 255.;
            vec4(r, g, b, 1.)
        }
        9 => {
            let r = u8::from_str_radix(&hex[1..3], 16).unwrap() as f32 / 255.;
            let g = u8::from_str_radix(&hex[3..5], 16).unwrap() as f32 / 255.;
            let b = u8::from_str_radix(&hex[5..7], 16).unwrap() as f32 / 255.;
            let a = u8::from_str_radix(&hex[7..9], 16).unwrap() as f32 / 255.;
            vec4(r, g, b, a)
        }
        _ => unreachable!()
    }
}
