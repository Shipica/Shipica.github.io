//! TODO rename this module maybe?
//!
//! Common is not the best name for it, but i don't know
//! what is better

use web_sys::DomMatrix;

use crate::{
    canvas::Canvas,
    math::{Matrix, Vec2},
};

use super::Widget;

// ----------------------------------------------------------------
// Transform
// ----------------------------------------------------------------
pub struct Transform<T>
where
    T: Widget,
{
    pub transform: Matrix,
    pub inner: T,
}

impl<T> Widget for Transform<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        canvas.transform(self.transform);
        self.inner.draw(canvas);
        canvas.transform(self.transform.inverse());
    }
}

// ----------------------------------------------------------------
// Fill
// ----------------------------------------------------------------
pub struct Fill<T>
where
    T: Widget,
{
    pub inner: T,
}

impl<T> Widget for Fill<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        self.inner.draw(canvas);
        canvas.fill();
    }
}

// ----------------------------------------------------------------
// Stroke
// ----------------------------------------------------------------
pub struct Stroke<T>
where
    T: Widget,
{
    pub inner: T,
}

impl<T> Widget for Stroke<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        self.inner.draw(canvas);
        canvas.stroke();
    }
}

// ----------------------------------------------------------------
// Translate
// ----------------------------------------------------------------
pub struct Translate<T>
where
    T: Widget,
{
    pub translation: Vec2,
    pub inner: T,
}

impl<T> Widget for Translate<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        canvas.translate([self.translation.x, self.translation.y]);
        self.inner.draw(canvas);
        canvas.translate([-self.translation.x, -self.translation.y]);
    }
}

// ----------------------------------------------------------------
// Scale
// ----------------------------------------------------------------
pub struct Scale<T>
where
    T: Widget,
{
    pub scale: f64,
    pub inner: T,
}

impl<T> Widget for Scale<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        canvas.scale(self.scale);
        self.inner.draw(canvas);
        canvas.scale(1.0 / self.scale);
    }
}

// ----------------------------------------------------------------
// Rotate
// ----------------------------------------------------------------
pub struct Rotate<T>
where
    T: Widget,
{
    pub angle: f64,
    pub inner: T,
}

impl<T> Widget for Rotate<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        canvas.rotate(self.angle);
        self.inner.draw(canvas);
        canvas.rotate(-self.angle);
    }
}

pub struct Inspect<T, F>
where
    T: Widget,
    F: Fn(),
{
    pub inner: T,
    pub f: F,
}

impl<T, F> Widget for Inspect<T, F>
where
    T: Widget,
    F: Fn(),
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        if canvas.debug {
            (self.f)();
        }
        self.inner.draw(canvas);
    }
}
