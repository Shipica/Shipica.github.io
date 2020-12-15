use wasm_bindgen::JsValue;

use crate::{canvas::Canvas, math::Color};

pub fn node(context: &Canvas) {
    // shadow color
    context.set_shadow_color("#1B264F");
    context.set_shadow_blur(10.0);
    context.set_shadow_offset(0.0, 5.0);
    // background color;
    context.set_fill_style("#25232388");
    // node border width
    context.set_line_width(5.0 / 2.0);
    // node border color
    context.set_stroke_style("#F5F1ED");
}

pub fn connection(context: &Canvas) {
    context.set_stroke_style("#A99985");
    context.set_shadow_blur(3.0);
    context.set_line_width(4.0);
}

pub fn delete_line(context: &Canvas) {
    context.set_stroke_style("#FFFFFF70");
    context.set_fill_style("#FFFFFF90");
    context.render_context.set_font("12px sans-serif");

    // context.context.set_line_dash((10.0, 10.0).into());
    context.set_line_width(4.0);
    context.set_line_cap("round");
}

pub fn dot(context: &Canvas) {
    // input dot color
    context.set_fill_style("#DAD2BC");
    // shadow color
    context.set_shadow_color("#1B264F");
    context.set_shadow_blur(4.0);
    context.set_shadow_offset(0.0, 0.0);
    context.set_line_width(1.0);
}
