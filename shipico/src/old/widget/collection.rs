use std::iter::FromIterator;

use crate::canvas::Canvas;

use super::Widget;

// NOTE: For all collections inner collection field must be named `body`
// for better readability.

#[derive(Default)]
pub struct Stack<T> {
    pub body: Vec<T>,
}

// impl<T> Widget for Stack<T>
// where
//     T: IntoIterator + Clone,
//     T::Item: Widget,
// {
//     #[inline]
//     fn draw(&self, canvas: &mut Canvas) {
//         self.body.clone().into_iter().for_each(|x| x.draw(canvas));
//     }
// }

// impl<T> Stack<T>
// where
//     T: IntoIterator + Clone,
//     T::Item: Widget,
// {
//     #[inline]
//     pub fn of(body: T) -> Self {
//         Stack { body }
//     }
// }

impl<T> Widget for Stack<T>
where
    T: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        self.body.iter().for_each(|x| x.draw(canvas));
    }
}

impl<T> Stack<T>
where
    T: Widget,
{
    #[inline]
    pub fn of<I>(body: I) -> Self
    where
        I: Iterator,
        I::Item: Widget,
        Vec<T>: FromIterator<I::Item>,
    {
        Stack {
            body: body.collect(),
        }
    }

    #[inline]
    pub fn from(body: Vec<T>) -> Self {
        Stack { body }
    }
}

pub struct StackBuilder<L, O>
where
    L: Fn(usize) -> Option<O>,
    O: Widget,
{
    pub lambda: L,
}

impl<L, O> Widget for StackBuilder<L, O>
where
    L: Fn(usize) -> Option<O>,
    O: Widget,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        let mut counter = 0;
        while let Some(widget) = (self.lambda)(counter) {
            widget.draw(canvas);
            counter += 1;
        }
    }
}

impl<L, O> StackBuilder<L, O>
where
    L: Fn(usize) -> Option<O>,
    O: Widget,
{
    #[inline]
    pub fn of(lambda: L) -> StackBuilder<L, O> {
        StackBuilder { lambda }
    }
}

// TODO: List, something else
