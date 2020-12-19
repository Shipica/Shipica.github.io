mod collection;
mod common;
mod shape;
mod style;

pub use collection::*;
pub use common::*;
pub use shape::*;
pub use style::*;

use crate::{
    canvas::Canvas,
    math::{Matrix, Vec2},
};

// Tips for implementing Widget:
// - Prefer simple structs, maybe even tuple structs if possible.
// - Prefer types that implement Default trait.
// - ...TBA
pub trait Widget {
    // Q: Should widgets implement bound_rect?
    // fn bound_rect(&self) -> Rect;

    // Q: Should widgets handle events?
    // fn handle_event(&mut self, event: InputEvent) -> bool;

    fn draw(&self, canvas: &mut Canvas);

    #[inline]
    fn as_dyn(&self) -> &dyn Widget
    where
        Self: Sized,
    {
        self as &dyn Widget
    }

    #[inline]
    fn boxed(self) -> Box<dyn Widget>
    where
        Self: Sized + 'static,
    {
        Box::new(self)
    }

    #[inline]
    fn filled(self) -> Fill<Self>
    where
        Self: Sized,
    {
        Fill { inner: self }
    }

    #[inline]
    fn stroked(self) -> Stroke<Self>
    where
        Self: Sized,
    {
        Stroke { inner: self }
    }

    #[inline]
    fn translated(self, translation: impl Into<Vec2>) -> Translate<Self>
    where
        Self: Sized,
    {
        Translate {
            inner: self,
            translation: translation.into(),
        }
    }

    #[inline]
    fn rotated(self, angle: impl Into<f64>) -> Rotate<Self>
    where
        Self: Sized,
    {
        Rotate {
            inner: self,
            angle: angle.into(),
        }
    }

    #[inline]
    fn scaled(self, scale: impl Into<f64>) -> Scale<Self>
    where
        Self: Sized,
    {
        Scale {
            inner: self,
            scale: scale.into(),
        }
    }

    #[inline]
    fn transformed(self, transform: impl Into<Matrix>) -> Transform<Self>
    where
        Self: Sized,
    {
        Transform {
            inner: self,
            transform: transform.into(),
        }
    }

    fn inspect<F>(self, f: F) -> Inspect<Self, F>
    where
        F: Fn(),
        Self: Sized,
    {
        Inspect { inner: self, f }
    }
}

impl Widget for Box<dyn Widget> {
    fn draw(&self, canvas: &mut Canvas) {
        (**self).draw(canvas)
    }
}

impl Widget for &dyn Widget {
    fn draw(&self, canvas: &mut Canvas) {
        (**self).draw(canvas)
    }
}

pub trait Component {
    fn build(&self) -> Box<dyn Widget>;
}
