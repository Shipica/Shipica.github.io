use crate::canvas::Canvas;
use crate::Widget;

/// Helper macros to generate simpliest style widgets that are supported by canvas "out of the box",
/// like `shadow_color`, `fill_color` etc.
macro_rules! styles {
    {$($style: ident($($param:ident : $ty:ty),*));*;} => {
        styles!($($style($($param : $ty),*));*);
    };
    {$($style: ident($($param:ident : $ty:ty),*));*} => {
        ::paste::paste! {
            $(
                pub struct [<$style:camel>]<T> {
                    inner: T,
                    $(
                        $param: $ty
                    ),*
                }

                impl<T> Widget for [<$style:camel>]<T> where T: Widget {
                    #[inline]
                    fn draw(&self, canvas: &mut Canvas) {
                        if canvas.debug {
                            crate::log!("drawing {}", stringify!([<$style:camel>]));
                        }
                        canvas.[<set_ $style:snake>]($(self.$param),*);
                        // canvas.render_context.save();
                        self.inner.draw(canvas);
                        // canvas.render_context.restore();
                    }
                }
            )*


            pub trait WidgetStyleExt: Widget {
                $(
                    // `Inline` is an instruction to the compiler
                    // to insert code from this function into the place
                    // it was called.
                    //
                    // It's generally acceptable to inline functions 1-5 lines long
                    // not to spend time on function pointer dereference in runtime.
                    //
                    // Though if function is called super often, long, and used in many places
                    // it's probably better not to inline it.
                    //
                    // Tradeoff of inlining in compiled binary size vs runtime performance.
                    //
                    // Delete this after you read it.
                    #[inline]
                    #[doc = "Wrap `self` in `" $style:camel "` style widget."]
                    #[doc = "`canvas.set_" $style:snake "` with provided params will be called just before drawing `self`."]
                    fn [<with_ $style:snake>](self, $($param: impl Into<$ty>),*) -> [<$style:camel>]<Self>
                        where Self: Sized
                    {
                        [<$style:camel>] {
                            inner: self,
                            $(
                                $param: $param.into()
                            ),*
                        }
                    }
                )*
            }

            impl<T> WidgetStyleExt for T where T: Widget {}
        }
    }
}

styles! {
    shadow_color(color: &'static str);
    shadow_blur(blur: f64);
    shadow_offset(x: f64, y: f64);

    fill_style(style: &'static str);
    stroke_style(style: &'static str);

    line_width(width: f64);

    font(font: &'static str);

    line_cap(cap: &'static str);
}
