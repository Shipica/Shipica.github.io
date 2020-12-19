#![feature(unsized_fn_params)]

use glam::{vec2, vec3, Mat3, Mat4, Quat, Vec2};
use miniquad::*;
use obj::{load_obj, Obj};
use std::io::BufReader;
use std::{fs::File, io::Cursor};

mod shader;
use shader::*;

struct Input {
    mouse_down: bool,
    last_mouse_pos: Vec2,
}

struct Stage {
    pipeline: Pipeline,
    bindings: Bindings,
    index_count: usize,
    transform: Mat3,
    position: Vec2,
    scale: f32,
    input: Input,
}

const PERFECT_SIZE: (f32, f32) = (1000., 1000.);

impl Stage {
    // @NOTE:
    // This function will be executed once at the start of the program.
    // Right after the `main` is called.
    pub fn new(ctx: &mut Context) -> Stage {
        let (w, h) = ctx.screen_size();

        // // Include F.obj model bytes into the binary
        // let model = Cursor::new(include_bytes!("../assets/F.obj"));
        // let input = BufReader::new(model);
        // // Parse it into model information
        // let model: Obj = load_obj(input).unwrap();

        // // Create vertex buffer from obj vertices
        // let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &model.vertices);
        // // Create index buffer from obj indices
        // let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &model.indices);
        // // Create framework structure for the model.
        // // `Bindings` are the struct that represents model information.
        // // It can be transferred to the shader at the moment of rendering.
        // let bindings = Bindings {
        //     vertex_buffers: vec![vertex_buffer.clone()],
        //     index_buffer: index_buffer.clone(),
        //     images: vec![],
        // };

        let bindings = node(ctx);
        let index_count = bindings.index_buffer.size() / 2;
        // Create Shader program
        let shader = Shader::new(
            ctx,
            offscreen_shader::VERTEX,
            offscreen_shader::FRAGMENT,
            offscreen_shader::meta(),
        )
        .unwrap();

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("a_position", VertexFormat::Float2),
                // VertexAttribute::new("a_norm", VertexFormat::Float3),
            ],
            shader,
            PipelineParams {
                primitive_type: PrimitiveType::Triangles,
                cull_face: CullFace::Nothing,
                ..Default::default()
            },
        );

        let w_ = w / PERFECT_SIZE.0;
        let h_ = h / PERFECT_SIZE.1;

        Stage {
            pipeline,
            bindings,
            index_count,
            input: Input {
                mouse_down: false,
                last_mouse_pos: Vec2::zero(),
            },
            position: Vec2::zero(),
            scale: 1.,
            transform: m3::identity() * m3::projection(w_, h_),
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
            self.transform = m3::translation(-delta) * self.transform;
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
        self.transform = m3::scaling(zoom) * self.transform;
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        // On resize it's nessesary to do some math to rescale projection.
        //
        // Current self.transform already contains projection information, so you have to
        // first remove it like `self.transform * m3::projection(1. / old_w, 1. / old_h)`.
        //
        // `old_w` and `old_h` not like really old width and height. It's relation of old width
        // and height to the perfect screen size (`PERFECT_SIZE`). It's calculated like `screen_w / PERFECT_SIZE.0`.
        //
        // After old projection is removed it's time to calculate new one.
        // `self.transform * m3::projection(new_w, new_h)`
        // `new_w` and `new_h` are relations of current width and height to the perfect screen size.
        //
        // This should work i hope.
    }

    fn draw(&mut self, ctx: &mut Context) {
        // Make our camera view matrix smaller for `F` model to fit in the screen
        let mvp = self.transform * m3::scaling(0.1);

        // Clear color buffer with white color
        ctx.begin_default_pass(PassAction::Clear {
            color: Some((1., 1., 1., 1.)),
            depth: None,
            stencil: None,
        });
        // Prepare shaders (gl.useProgram), set face culling, depth tests and such shit
        ctx.apply_pipeline(&self.pipeline);
        // Push vertices, indices and textures of the model to the shader
        ctx.apply_bindings(&self.bindings);
        // Push transform matrix to the uniforms of the shader
        ctx.apply_uniforms(&offscreen_shader::Uniforms { mvp });
        // Draw 1 instance of the model containing 12 triangles (36 indices) of the first (0) model in the bindings
        ctx.draw(0, self.index_count as i32, 1);
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
}

use lyon::math::{rect, Point};
use lyon::tessellation::basic_shapes::*;
use lyon::tessellation::geometry_builder::simple_builder;
use lyon::tessellation::{FillOptions, VertexBuffers};

fn node(ctx: &mut Context) -> Bindings {
    let mut geometry: VertexBuffers<Point, u16> = VertexBuffers::new();

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
    let options = FillOptions::tolerance(0.05);

    fill_rounded_rectangle(
        &rect(0.0, 0.0, 200.0, 100.0),
        &BorderRadii::new_all_same(10.),
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
}
