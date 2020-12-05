use std::collections::VecDeque;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::*;

use crate::math::{Matrix, Point, Rect, Vec2};

pub struct Canvas {
    pub window: Window,
    pub canvas_element: HtmlCanvasElement,
    pub render_context: CanvasRenderingContext2d,
    pub transform: Matrix,
    transform_stack: VecDeque<Matrix>,
    pub debug: bool,
}

impl Canvas {
    pub fn new() -> Canvas {
        let window = web_sys::window().expect("no global `window` exists");
        let document = window.document().expect("no global `document` exists");

        let canvas_element: HtmlCanvasElement =
            if let Some(canvas) = document.get_element_by_id("canvas") {
                canvas
            } else {
                let canvas = document.create_element("canvas").unwrap();
                document
                    .body()
                    .expect("document should have a body")
                    .append_child(&canvas)
                    .unwrap();

                canvas
            }
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let render_context: CanvasRenderingContext2d = canvas_element
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let mut canvas = Canvas {
            window,
            canvas_element,
            transform: Default::default(),
            render_context,
            transform_stack: Default::default(),
            debug: false,
        };

        canvas.reset_canvas_size();

        canvas
    }

    pub fn reset_canvas_size(&mut self) {
        let width = self.window.inner_width().unwrap().as_f64().unwrap() as u32;
        let height = self.window.inner_height().unwrap().as_f64().unwrap() as u32;
        self.canvas_element.set_width(width);
        self.canvas_element.set_height(height);
    }

    pub fn is_rect_in_screen(&self, rect: Rect) -> bool {
        let transformed = Rect::from_center_size(
            self.transform.transform_point(rect.center()),
            (
                rect.size().width * self.transform.a,
                rect.size().height * self.transform.d,
            ),
        );
        self.screen_rect().overlaps(&transformed)
    }

    fn screen_rect(&self) -> Rect {
        let size = (
            self.canvas_element.width() as f64,
            self.canvas_element.height() as f64,
        );
        let center = (
            self.canvas_element.width() as f64 / 2.0,
            self.canvas_element.height() as f64 / 2.0,
        );
        Rect::from_center_size(center, size)
    }

    /// Begin actions to draw figure
    /// Each call will erase previous context and start draw from scratch
    pub fn begin_path(&self) {
        self.render_context.begin_path()
    }

    /// Move cursor to the new position
    pub fn move_to(&self, point: impl Into<Point>) {
        let point = self.transform.transform_point(point);
        self.render_context.move_to(point.x, point.y);
    }

    /// Draw and arc with `radius` from current point to `point`.
    ///
    /// `start_angle` and `end_angle` are in radians
    pub fn arc(&self, point: impl Into<Point>, radius: f64, start_angle: f64, end_angle: f64) {
        let point = self.transform.transform_point(point);
        let radius = radius * self.transform.a;
        self.render_context
            .arc(point.x, point.y, radius, start_angle, end_angle)
            .unwrap();
    }

    /// Draw line from current point to new point
    pub fn line_to(&self, point: impl Into<Point>) {
        let point = self.transform.transform_point(point);
        self.render_context.line_to(point.x, point.y);
    }

    /// Finish draw by connecting outlining each point
    pub fn stroke(&self) {
        self.render_context.stroke();
    }

    /// Finish draw by filling figure with color
    pub fn fill(&self) {
        self.render_context.fill();
    }

    /// Tries to finish drawing by connecting last point and first point with line
    pub fn close_path(&self) {
        self.render_context.close_path();
    }

    /// Draw empty rect
    pub fn stroke_rect(&self, rect: impl Into<Rect>) {
        let rect = rect.into();
        self.render_context.stroke_rect(
            rect.center().x - rect.size().width / 2.0,
            rect.center().y - rect.size().height / 2.0,
            rect.size().width,
            rect.size().height,
        )
    }

    /// Dtaw filled rect
    pub fn fill_rect(&self, rect: impl Into<Rect>) {
        let rect = rect.into();
        self.render_context.fill_rect(
            rect.center().x,
            rect.center().y,
            rect.size().width,
            rect.size().height,
        )
    }

    /// Crear rectangular area
    pub fn clear_rect(&self, rect: impl Into<Rect>) {
        let rect = rect.into();
        self.render_context.clear_rect(
            rect.center().x,
            rect.center().y,
            rect.size().width,
            rect.size().height,
        )
    }

    pub fn set_font(&self, font: &str) {
        self.render_context.set_font(font);
    }

    pub fn set_fill_style(&self, style: &str) {
        self.render_context
            .set_fill_style(&JsValue::from_str(style));
    }

    pub fn set_stroke_style(&self, style: &str) {
        self.render_context
            .set_stroke_style(&JsValue::from_str(style));
    }

    pub fn set_line_cap(&self, cap: &str) {
        self.render_context.set_line_cap(cap);
    }

    pub fn set_shadow_color(&self, color: &str) {
        self.render_context.set_shadow_color(color);
    }

    pub fn set_shadow_blur(&self, blur: f64) {
        let blur = blur * self.transform.a;
        self.render_context.set_shadow_blur(blur);
    }

    pub fn set_shadow_offset(&self, x: f64, y: f64) {
        let x = x * self.transform.a;
        let y = y * self.transform.a;
        self.render_context.set_shadow_offset_x(x);
        self.render_context.set_shadow_offset_y(y);
    }

    pub fn set_line_width(&self, width: f64) {
        let width = width * self.transform.a;
        self.render_context.set_line_width(width);
    }

    pub fn quadratic_curve_to(&self, anchor_1: impl Into<Point>, point: impl Into<Point>) {
        let anchor_1 = self.transform.transform_point(anchor_1);
        let point = self.transform.transform_point(point);
        self.render_context
            .quadratic_curve_to(anchor_1.x, anchor_1.y, point.x, point.y)
    }

    pub fn bezier_curve_to(
        &self,
        anchor_1: impl Into<Point>,
        anchor_2: impl Into<Point>,
        point: impl Into<Point>,
    ) {
        let anchor_1 = self.transform.transform_point(anchor_1);
        let anchor_2 = self.transform.transform_point(anchor_2);
        let point = self.transform.transform_point(point);
        self.render_context.bezier_curve_to(
            anchor_1.x, anchor_1.y, anchor_2.x, anchor_2.y, point.x, point.y,
        );
    }

    pub fn reset(&mut self) {
        self.render_context.reset_transform().unwrap();
        let width = self.canvas_element.width();
        let height = self.canvas_element.height();

        self.render_context
            .clear_rect(0.0, 0.0, width as f64, height as f64);
        // TODO styles
        self.render_context
            .set_fill_style(&JsValue::from_str("#70798c"));
        self.render_context
            .fill_rect(0.0, 0.0, width as f64, height as f64);
    }

    pub fn get_transform(&self) -> Matrix {
        self.transform
    }

    pub fn translate(&mut self, delta: impl Into<Vec2>) {
        self.transform = Matrix::translation(delta) * self.transform;
    }

    pub fn rotate(&mut self, angle: impl Into<f64>) {
        self.transform = Matrix::rotation(angle.into(), (0.0, 0.0)) * self.transform;
    }

    pub fn scale(&mut self, scale: impl Into<Vec2>) {
        self.transform =
            Matrix::scaling(scale, (-self.transform.x, -self.transform.y)) * self.transform;
    }

    pub fn transform(&mut self, transform: Matrix) {
        self.transform = transform * self.transform;
    }

    pub fn save_transform(&mut self) {
        self.transform_stack.push_front(self.transform);
    }

    pub fn reset_transform(&mut self) {
        if let Some(transform) = self.transform_stack.pop_front() {
            self.transform = transform;
        }
    }
}
