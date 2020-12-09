use miniquad::*;

use glam::{vec2, vec3, vec4, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

mod shader;
use shader::*;

struct Input {
    mouse_down: bool,
    last_mouse_pos: Vec2,
}

struct Stage {
    post_processing_pipeline: Pipeline,
    post_processing_bind: Bindings,
    offscreen_pipeline: Pipeline,
    offscreen_bind: Bindings,
    offscreen_pass: RenderPass,
    transform: Mat4,
    // proj: Mat4,
    input: Input,
}

const PERFECT_SIZE: (f32, f32) = (1000., 1000.);

impl Stage {
    pub fn new(ctx: &mut Context) -> Stage {
        let (w, h) = ctx.screen_size();
        let color_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: w as _,
                height: h as _,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let depth_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: w as _,
                height: h as _,
                format: TextureFormat::Depth,
                ..Default::default()
            },
        );

        let offscreen_pass = RenderPass::new(ctx, color_img, None);

        // Plane
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos         color     */
            -0.5, -0.5,    1.0, 0.0, 0.0, 1.0,
             0.5, -0.5,    0.0, 1.0, 0.0, 1.0,
             0.5,  0.5,    0.0, 0.0, 1.0, 1.0,
            -0.5,  0.5,    0.5, 0.5, 0.5, 1.0,
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        #[rustfmt::skip]
        let indices: &[u16] = &[
             0, 1, 2, 0, 2, 3,
        ];

        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let offscreen_bind = Bindings {
            vertex_buffers: vec![vertex_buffer.clone()],
            index_buffer: index_buffer.clone(),
            images: vec![],
        };

        // Post process plane
        #[rustfmt::skip]
        let vertices: &[f32] = &[
            /* pos         uvs */
            -1.0, -1.0,    0.0, 0.0,
             1.0, -1.0,    1.0, 0.0,
             1.0,  1.0,    1.0, 1.0,
            -1.0,  1.0,    0.0, 1.0,
        ];

        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices: &[u16] = &[0, 1, 2, 0, 2, 3];

        let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let post_processing_bind = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![color_img],
        };

        let default_shader = Shader::new(
            ctx,
            post_processing_shader::VERTEX,
            post_processing_shader::FRAGMENT,
            post_processing_shader::meta(),
        )
        .unwrap();

        let post_processing_pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            default_shader,
        );

        let offscreen_shader = Shader::new(
            ctx,
            offscreen_shader::VERTEX,
            offscreen_shader::FRAGMENT,
            offscreen_shader::meta(),
        )
        .unwrap();

        let offscreen_pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float2),
                VertexAttribute::new("color0", VertexFormat::Float4),
            ],
            offscreen_shader,
            PipelineParams {
                primitive_type: PrimitiveType::Triangles,
                ..Default::default()
            },
        );

        let w_ = w / PERFECT_SIZE.0;
        let h_ = h / PERFECT_SIZE.1;

        Stage {
            post_processing_pipeline,
            post_processing_bind,
            offscreen_pipeline,
            offscreen_bind,
            offscreen_pass,
            input: Input {
                mouse_down: false,
                last_mouse_pos: Vec2::zero(),
            },
            transform: Mat4::identity()
                * Mat4::from_translation(vec3(0., 0., 1.))
                * Mat4::from_scale(vec3(1., 1., 1.)),
        }
    }
}

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
        let (w, h) = ctx.screen_size();
        let mouse_pos = screen_to_local(self.input.last_mouse_pos, vec2(w, h));
        // let mouse_pos = vec2(0.25, -0.25);
        // let canvas_pos = self.camera.position * self.camera.zoom;
        // let world_position = (mouse_pos - canvas_pos) / self.camera.zoom;

        let mouse_pos = vec2(0.25, -0.25);
        // let mouse_pos = self.input.last_mouse_pos.extend(0.);
        let inversed = self.transform.inverse();
        let world_position = inversed.transform_vector3(mouse_pos.extend(0.));

        info!(
            "
            mouse pos: {}
            mouse world pos: x: {:1.10}, y: {:1.10}",
            mouse_pos, world_position.x, world_position.y
        );

        // x = self.x() - x;
        // y = self.y() - y;

        // x /= self.scale();
        // y /= self.scale();
        // let delta_vec = self.transform.transform_vector3(vec3(x, y, 0.))/*  + vec3(x, y, 0.) */;

        // let mouse_point = self.transform.transform_point3(vec3(x, y, 0.0));
        // let mouse_point = self.transform.transform_point3(vec3(x, y, 0.)) - vec3(x, y, 0.) * zoom;
        // info!("mouse_pos: x:{}, y:{}", delta_vec.x, delta_vec.y);
        // info!("mouse_pos: x:{:1.5}, y:{:1.5}", x, y);

        // let mouse_point = self.transform.transform_point3(vec2(-x, -y).extend(0.));
        // info!("mouse_pos: x:{}, y:{}", mouse_point.x, mouse_point.y);

        // self.transform = self.transform * Mat4::from_rotation_z(0.03);
    }

    // fn mouse_button_down_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _button: MouseButton,
    //     _x: f32,
    //     _y: f32,
    // ) {
    //     self.input.mouse_down = true;
    // }

    // fn mouse_button_up_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _button: MouseButton,
    //     _x: f32,
    //     _y: f32,
    // ) {
    //     self.input.mouse_down = false;
    // }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        // if self.input.mouse_down {
        //     let screen_size = ctx.screen_size();
        //     let mut delta = self.input.last_mouse_pos - vec2(x, y);
        //     delta = vec2(-delta.x / screen_size.0, delta.y / screen_size.1);
        //     delta = delta * 2.0 / self.scale();
        //     self.camera.position += delta;
        // }
        self.input.last_mouse_pos = vec2(x, y);
    }

    fn mouse_wheel_event(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let zoom = if y > 0. { 1.05 } else { 0.95 };
        let (w, h) = ctx.screen_size();

        // mouse position in normal space
        let mouse_pos = screen_to_local(self.input.last_mouse_pos, vec2(w, h));

        // let mouse_pos = vec3(0.25, -0.25, 1.);
        let mut inversed = self.transform.inverse();
        let world_position = inversed.transform_vector3(mouse_pos.extend(1.));

        // info!(
        //     "mouse world pos: x: {:1.10}, y: {:1.10}",
        //     world_position.x, world_position.y
        // );

        // info!("before: {:#?}", self.transform);
        self.transform = self.transform * Mat4::from_translation(mouse_pos.extend(1.));
        self.transform = self.transform * Mat4::from_scale(vec3(zoom, zoom, 1.));
        self.transform = self.transform * Mat4::from_translation(-mouse_pos.extend(1.));
        // info!("after: {:#?}", self.transform);
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        // calculating relationship of the current screen size to the perfect screen size
        // it is used to construct new orto matrix that won't deform any existing positions
        // let w_ = width / PERFECT_SIZE.0;
        // let h_ = height / PERFECT_SIZE.1;
        // let proj = Mat4::orthographic_rh_gl(-w_, w_, -h_, h_, 0.01, 10.0);

        // // calculating how much screen should move back to negate projection change postition slide
        // // we want (0.0, 0.0) before resize and (0.0, 0.0) after resize to be in the same point.
        // let slide = vec2(
        //     (width - PERFECT_SIZE.0) / PERFECT_SIZE.0,
        //     (height - PERFECT_SIZE.1) / PERFECT_SIZE.1,
        // ) / 4.0;
        // self.transform = proj * Mat4::from_translation(slide.extend(-1.));

        let color_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::RGBA8,
                ..Default::default()
            },
        );
        let depth_img = Texture::new_render_texture(
            ctx,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::Depth,
                ..Default::default()
            },
        );

        let offscreen_pass = RenderPass::new(ctx, color_img, depth_img);

        self.offscreen_pass.delete(ctx);
        self.offscreen_pass = offscreen_pass;
        self.post_processing_bind.images[0] = color_img;
    }

    fn draw(&mut self, ctx: &mut Context) {
        let (w, h) = ctx.screen_size();
        let w_ = w / PERFECT_SIZE.0;
        let h_ = h / PERFECT_SIZE.1;

        // let transform = Mat4::from_scale_rotation_translation(
        //     vec3(self.camera.zoom, self.camera.zoom, 1.),
        //     Quat::default(),
        //     self.camera.position.extend(1.),
        // );
        // * Mat4::orthographic_rh_gl(-w_, w_, -h_, h_, 0.01, 10.);

        // the offscreen pass, rendering an rotating, untextured cube into a render target image
        ctx.begin_pass(
            self.offscreen_pass,
            PassAction::clear_color(1.0, 1.0, 1.0, 1.0),
        );
        ctx.apply_pipeline(&self.offscreen_pipeline);
        ctx.apply_uniforms(&self.offscreen_bind);
        ctx.apply_bindings(&self.offscreen_bind);
        ctx.apply_uniforms(&offscreen_shader::Uniforms {
            // mvp: self.transform * Mat4::orthographic_rh_gl(-w, w, -h, h, 0.01, 10.),
            mvp: self.transform,
        });
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();

        ctx.begin_default_pass(PassAction::Nothing);
        ctx.apply_pipeline(&self.post_processing_pipeline);
        ctx.apply_bindings(&self.post_processing_bind);
        ctx.apply_uniforms(&post_processing_shader::Uniforms {
            resolution: glam::vec2(w, h),
        });
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn main() {
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Stage::new(&mut ctx), ctx)
    });
}

trait AsShape {
    fn vertices(&self) -> &[Vec2];
    fn color(&self) -> Vec4;
}

struct Line {}
