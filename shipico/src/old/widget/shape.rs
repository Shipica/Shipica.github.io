use crate::{
    canvas::Canvas,
    math::{Ellipse, Line, Rect, RoundedRect},
};

use super::Widget;

// NOTE: Shapes must not contain any inner widgets, as it's simplest possible primitives

impl Shape for Line {
    #[inline]
    fn outline(&self, canvas: &mut Canvas) {
        canvas.move_to(self.start);
        canvas.line_to(self.end);
    }
    #[inline]
    fn bound_rect(&self) -> Rect {
        Rect::from_points(self.start, self.end)
    }
}

impl Shape for RoundedRect {
    fn outline(&self, canvas: &mut Canvas) {
        let left = self.rect.left;
        let right = self.rect.right;
        let top = self.rect.top;
        let bottom = self.rect.bottom;

        let top_left_line_point = (left + self.radius_x, top);
        let top_right_line_point = (right - self.radius_x, top);

        let right_top_line_point = (right, top + self.radius_y);
        let right_bottom_line_point = (right, bottom - self.radius_y);

        let bottom_right_line_point = (right - self.radius_x, bottom);
        let bottom_left_line_point = (left + self.radius_x, bottom);

        let left_bottom_line_point = (left, bottom - self.radius_y);
        let left_top_line_point = (left, top + self.radius_y);

        let left_top_anchor = (left, top);
        let right_top_anchor = (right, top);
        let left_bottom_anchor = (left, bottom);
        let right_bottom_anchor = (right, bottom);

        //         crate::log!(
        //             "

        //                           {:3.1?}             {:3.1?}
        //                                __________________________
        //           {:3.1?}    ___/                          \\___     {:3.1?}
        //                         __/                                  \\__
        //                     ___/                                        \\___
        //                  __/                                                \\__
        // {:3.1?} |                                                      |  {:3.1?}
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        //                 |                                                      |
        // {:3.1?}  |__                                                  __|  {:3.1?}
        //                    \\___                                          ___/
        //                        \\__                                    __/
        //         {:3.1?}    \\___                            ___/    {:3.1?}
        //                               \\__________________________/

        //                           {:3.1?}              {:3.1?}
        // ",
        //             top_left_line_point,
        //             top_right_line_point,
        //             left_top_anchor,
        //             right_top_anchor,
        //             left_top_line_point,
        //             right_top_line_point,
        //             left_bottom_line_point,
        //             right_bottom_line_point,
        //             left_bottom_anchor,
        //             right_bottom_anchor,
        //             bottom_left_line_point,
        //             bottom_right_line_point,
        //         );

        // drawing top line
        canvas.move_to(top_left_line_point);
        canvas.line_to(top_right_line_point);
        // drawing right top curve
        canvas.quadratic_curve_to(right_top_anchor, right_top_line_point);
        // drawing right line
        canvas.line_to(right_bottom_line_point);
        // drawing right borrom curve
        canvas.quadratic_curve_to(right_bottom_anchor, bottom_right_line_point);
        // drawing bottom line
        canvas.line_to(bottom_left_line_point);
        // drawing left bottom curve
        canvas.quadratic_curve_to(left_bottom_anchor, left_bottom_line_point);
        // drawing left line
        canvas.line_to(left_top_line_point);
        // drawing left top curve
        canvas.quadratic_curve_to(left_top_anchor, top_left_line_point);
    }
    #[inline]
    fn bound_rect(&self) -> Rect {
        self.rect
    }
}

impl Shape for Ellipse {
    #[inline]
    fn bound_rect(&self) -> Rect {
        Rect::from_center_half_extent(self.center, [self.radius_x, self.radius_y])
    }
    fn outline(&self, canvas: &mut Canvas) {
        let top_point = self.center + [0.0, self.radius_y];
        let bottom_point = self.center + [0.0, -self.radius_y];
        let left_point = self.center + [-self.radius_x, 0.0];
        let right_point = self.center + [self.radius_x, 0.0];

        let left_top_anchor = self.center + [-self.radius_x, self.radius_y];
        let right_top_anchor = self.center + [self.radius_x, self.radius_y];
        let left_bottom_anchor = self.center + [-self.radius_x, -self.radius_y];
        let right_bottom_anchor = self.center + [self.radius_x, -self.radius_y];

        canvas.move_to(left_point);
        canvas.quadratic_curve_to(left_top_anchor, top_point);
        canvas.quadratic_curve_to(right_top_anchor, right_point);
        canvas.quadratic_curve_to(right_bottom_anchor, bottom_point);
        canvas.quadratic_curve_to(left_bottom_anchor, left_point);
    }
}

// ----------------------------------------------------------------
// KEEP TRAIT AT THE BOTTOM PLEASE
// ----------------------------------------------------------------

pub trait Shape {
    fn outline(&self, canvas: &mut Canvas);
    fn bound_rect(&self) -> Rect;
}

impl<T> Widget for T
where
    T: Shape,
{
    #[inline]
    fn draw(&self, canvas: &mut Canvas) {
        canvas.begin_path();
        if canvas.is_rect_in_screen(self.bound_rect()) {
            self.outline(canvas)
        }
        canvas.close_path();
    }
}
