#![feature(type_alias_impl_trait)]
#![feature(associated_type_defaults)]
#![allow(unused_unsafe)]

use input::input;
use math::{AsLine, Point, Rect, Vec2};

use canvas::Canvas;
use tree::{NodeId, SocketId, SocketKind};
use ui::*;
use wasm_bindgen::{prelude::*, JsCast};

mod canvas;
mod capabilities;
mod function;
mod input;
mod math;
mod params;
mod temp_styles;
mod tree;
mod ui;
mod widget;

use function::*;
use params::*;
use web_sys::Event;
pub use widget::{Shape, Widget, WidgetStyleExt};

#[macro_export]
macro_rules! log {
    ($($t: tt)*) => {
        // the fuckin RA complains that log_1 is unsafe
        // so i wrapped it in unsafe block
        // Now it complains that unsafe is unnecessary.
        // Fuck you, RA
        unsafe{ web_sys::console::log_1(&format_args!($($t)*).to_string().into()) }
    }
}

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub struct Settings {
    delete_key_code: String,
    menu_key_code: String,
    zoom_speed: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            delete_key_code: "KeyX".to_string(),
            menu_key_code: "Space".to_string(),
            zoom_speed: 1.0,
        }
    }
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // let w = Rect::default()
    //     .with_shadow_color("#000")
    //     .with_shadow_blur(5.0);

    // disable right click context menu on canvas
    set_on_contextmenu(|event| {
        let event = event.dyn_into::<Event>().unwrap();
        event.prevent_default();
    });

    // resize canvas width and height according to new window dimensions
    set_on_resize(|x| {
        ui().canvas.reset_canvas_size();
    });

    set_on_keydown(|x| {
        input().on_key_down(x);
        // match x.code() {
        //     key if key == ui().settings.delete_key_code => {
        //         ui().state.delete_button = true;
        //     }
        //     key if key == ui().settings.menu_key_code => {
        //         if !ui().state.space_button {
        //             ui().floating_window.position = ui().state.mouse_pos;
        //             ui().state.space_button = true;
        //         }
        //     }
        //     _ => {}
        // }
    });

    set_on_keyup(|x| {
        input().on_key_up(x);
        // match x.code() {
        //     key if key == ui().settings.delete_key_code => {
        //         ui().state.delete_button = false;
        //     }
        //     key if key == ui().settings.menu_key_code => {
        //         ui().state.space_button = false;
        //         ui().floating_window.selected = None;
        //     }
        //     _ => {}
        // }
    });

    set_on_wheel(|x| {
        // input().on_wheel(x);

        // if ui().state.space_button || ui().state.delete_button {
        //     return;
        // }

        const ZOOM_SPEED: f64 = -0.05;
        let delta = if x.delta_y() > 0.0 {
            ZOOM_SPEED * ui().settings.zoom_speed
        } else {
            -ZOOM_SPEED * ui().settings.zoom_speed
        };
        let pivot = (x.x() as f64, x.y() as f64);

        ui().tree.zoom(delta, pivot.into());
        ui().redraw();
    });

    set_on_mouseup(|x| {
        input().on_mouse_up(x);

        // ui().state.mouse_down = false;
        // if ui().state.space_button {
        // } else if ui().state.delete_button {
        //     for cast_res in ui().tree.line_cast(ui().state.cut_line.unwrap()) {
        //         if let CastResult::Connection(input_socket) = cast_res {
        //             ui().tree.delete_connection(input_socket)
        //         }
        //     }
        // } else {
        //     match ui().state.action {
        //         Action::DragScreen => {}
        //         Action::DragNode(_) => {}
        //         Action::DragSocket(from) => {
        //             if let CastResult::Socket(to, _) = ui().tree.point_cast(ui().state.mouse_pos) {
        //                 ui().tree.create_connection(from, to);
        //             }
        //         }
        //         Action::None => {}
        //     }
        //     ui().state.action = Action::None;
        // }

        // ui().state.cut_line = None;
        // ui().state.phantom_connection = None;
    });

    set_on_mousemove(|x| {
        input().on_mouse_move(x.clone());

        // let mouse_pos = (x.x() as f64, x.y() as f64).into();
        // ui().state.mouse_pos = mouse_pos;
        // let mouse_delta: Vec2 = [x.movement_x() as f64, x.movement_y() as f64].into();
        // ui().state.mouse_delta = mouse_delta;

        // if ui().state.delete_button {
        //     if let Some(cut_line) = &mut ui().state.cut_line {
        //         cut_line.end = mouse_pos;
        //     }
        // } else if ui().state.space_button {
        //     ui().floating_window.on_mouse_move(mouse_pos);
        // } else {
        //     if ui().state.mouse_down {
        //         match ui().state.action {
        //             Action::DragScreen => {
        //                 ui().tree.drag(mouse_delta);
        //             }
        //             Action::DragNode(node_id) => {
        //                 ui().tree.drag_node(node_id, mouse_delta);
        //             }
        //             Action::DragSocket(_) => {
        //                 if let Some(phantom_connection) = &mut ui().state.phantom_connection {
        //                     phantom_connection.to = mouse_pos;
        //                 }
        //             }
        //             Action::None => {}
        //         }
        //     }
        // }
    });

    set_on_mousedown(|x| {
        input().on_mouse_down(x);

        // ui().state.mouse_down = true;
        // if ui().state.delete_button {
        //     ui().state.cut_line = Some(CutLine {
        //         start: ui().state.mouse_pos,
        //         end: ui().state.mouse_pos,
        //     });
        // } else if ui().state.space_button {
        //     ui().floating_window.on_click(ui);
        //     ui().state.action = Action::None;
        // } else {
        //     let res = ui().tree.point_cast(ui().state.mouse_pos);
        //     ui().state.action = match res {
        //         CastResult::Node(id) => Action::DragNode(id),
        //         CastResult::Socket(id, socket_pos) => {
        //             ui().state.phantom_connection = Some(PhantomConnection {
        //                 from: socket_pos,
        //                 to: ui().state.mouse_pos,
        //             });
        //             Action::DragSocket(id)
        //         }
        //         CastResult::Connection(id) => Action::DragScreen,
        //         CastResult::None => Action::DragScreen,
        //     };
        // }
    });

    ui().tree.create_node(
        FunctionDefinition {
            inputs: &[],
            outputs: &[ParamType::f64],
            // outputs: &[],
            name: "input_geo",
        },
        (200.0, 200.0).into(),
    );

    ui().tree.create_node(
        FunctionDefinition {
            inputs: &[ParamType::f64, ParamType::f64],
            outputs: &[ParamType::f64],
            name: "boolean",
        },
        (300.0, 450.0).into(),
    );

    ui().tree.create_node(
        FunctionDefinition {
            inputs: &[ParamType::f64],
            outputs: &[],
            name: "output_geo",
        },
        (200.0, 700.0).into(),
    );

    for i in 0..10 {
        for k in 0..10 {
            ui().tree.create_node(
                FunctionDefinition {
                    inputs: &[ParamType::f64],
                    outputs: &[],
                    name: "output_geo",
                },
                (i as f64 * 200.0, k as f64 * 100.0).into(),
            );
        }
    }

    ui().tree
        .create_connection((0, 0, SocketKind::Output), (1, 1, SocketKind::Input));
    ui().tree
        .create_connection((0, 0, SocketKind::Output), (1, 0, SocketKind::Input));

    ui().canvas.debug = false;

    ui().redraw();
    ui().redraw();

    Ok(())
}

#[derive(Clone, Copy, Debug)]
enum Action {
    DragScreen,
    DragNode(NodeId),
    DragSocket(SocketId),
    None,
}

impl Default for Action {
    fn default() -> Self {
        Action::None
    }
}

#[derive(Default)]
pub struct InputState {
    mouse_down: bool,
    mouse_pos: Point,
    mouse_delta: Vec2,
    delete_button: bool,
    space_button: bool,
    action: Action,

    phantom_connection: Option<PhantomConnection>,
    cut_line: Option<CutLine>,
}

#[derive(Default)]
pub struct FloatingWindow {
    position: Point,
    selected: Option<usize>,
}

#[derive(Clone, Copy, Debug)]
struct PhantomConnection {
    from: Point,
    to: Point,
}

impl Widget for PhantomConnection {
    fn draw(&self, canvas: &mut Canvas) {
        temp_styles::connection(canvas);
        canvas.begin_path();
        canvas.move_to(self.from);
        canvas.line_to(self.to);
        canvas.stroke();
    }
}

#[derive(Clone, Copy, Debug)]
struct CutLine {
    start: Point,
    end: Point,
}

impl Widget for CutLine {
    fn draw(&self, canvas: &mut Canvas) {
        temp_styles::delete_line(canvas);
        canvas.begin_path();
        canvas.move_to(self.start);
        canvas.line_to(self.end);
        canvas.stroke();

        let text_x = self.start.x + (self.end.x - self.start.x) / 2.0;
        let text_y = self.start.y + (self.end.y - self.start.y) / 2.0;

        let x_axis_angle = (self.end.y - self.start.y).atan2(self.end.x - self.start.x);
        let half_pi = std::f64::consts::FRAC_PI_2;

        let rotate = if x_axis_angle > -half_pi && x_axis_angle < half_pi {
            x_axis_angle
        } else {
            x_axis_angle + std::f64::consts::PI
        };

        let context = &canvas.render_context;
        context
            .transform(1.0, 0.0, 0.0, 1.0, text_x, text_y)
            .unwrap();
        context.rotate(rotate).unwrap();
        context.scale(3.0, 3.0).unwrap();
        let text = "Delete";
        let text_offset = 5.0 * text.len() as f64 / 2.0;
        context.translate(-text_offset, -3.0).unwrap();
        context.set_font("7px sans-serif");
        context.fill_text(text, 0.0, 0.0).unwrap();
        context.translate(text_offset, 3.0).unwrap();
        context.scale(-3.0, -3.0).unwrap();
        context.rotate(-rotate).unwrap();
        context
            .transform(1.0, 0.0, 0.0, 1.0, -text_x, -text_y)
            .unwrap();
    }
}

impl FloatingWindow {
    const FUNCTION_H: f64 = 50.0;
    const FUNCTION_W: f64 = 200.0;

    fn draw(&self, context: &Canvas) {
        context.render_context.set_line_width(2.0);
        context.render_context.stroke_rect(
            self.position.x,
            self.position.y,
            Self::FUNCTION_W,
            Self::FUNCTION_H * function::FUNCTIONS.len() as f64,
        );
        context.set_fill_style("#9999");
        context.render_context.fill_rect(
            self.position.x,
            self.position.y,
            Self::FUNCTION_W,
            Self::FUNCTION_H * function::FUNCTIONS.len() as f64,
        );
        let font_size = Self::FUNCTION_W / 10.0;
        context.set_fill_style("#111");
        context.set_shadow_blur(0.0);
        context
            .render_context
            .set_font(&format!("{}px sans-serif", font_size,));

        let fill_text = |i: usize, text: &str| {
            if self.selected.map(|x| x == i).unwrap_or(false) {
                context.set_fill_style("#F00");
            } else {
                context.set_fill_style("#111");
            }
            context
                .render_context
                .fill_text(
                    text,
                    self.position.x + (Self::FUNCTION_W * 0.1),
                    self.position.y
                        + ((Self::FUNCTION_H * i as f64) + Self::FUNCTION_H / 2.0)
                        + font_size / 4.0,
                )
                .unwrap();
        };

        for i in 0..function::FUNCTIONS.len() {
            fill_text(i, function::FUNCTIONS[i].name);
        }
    }

    fn bound_rect(&self) -> Rect {
        Rect::from_center_size(
            (
                self.position.x + Self::FUNCTION_W / 2.0,
                self.position.y + (Self::FUNCTION_H * function::FUNCTIONS.len() as f64) / 2.0,
            ),
            (
                Self::FUNCTION_W,
                (Self::FUNCTION_H * function::FUNCTIONS.len() as f64),
            ),
        )
    }

    fn on_mouse_move(&mut self, pos: Point) {
        let bounding_rect = self.bound_rect();

        self.selected = None;

        if bounding_rect.contains_point(pos) {
            let y = pos.y;
            for i in 0..function::FUNCTIONS.len() {
                if i as f64 * Self::FUNCTION_H + self.position.y < y
                    && i as f64 * Self::FUNCTION_H + Self::FUNCTION_H + self.position.y > y
                {
                    self.selected = Some(i);
                    break;
                }
            }
        }
    }

    fn on_click(&self) {
        if let Some(selected) = self.selected {
            ui().tree
                .create_node(function::FUNCTIONS[selected].clone(), self.position);
        }
    }
}

impl AsLine for CutLine {
    fn start(&self) -> Point {
        self.start
    }

    fn end(&self) -> Point {
        self.end
    }
}

// x + key
// key
// Click
// Drag
//

// trait Widget {
//     fn apply_style(&self, context: &RenderContext);
//     fn outline(&self, context: &RenderContext);

//     fn stroke(&self, context: &RenderContext) {
//         self.apply_style(context);
//         context.begin_path();
//         self.outline(context);
//         context.stroke();
//     }

//     fn fill(&self, context: &RenderContext) {
//         self.apply_style(context);
//         context.begin_path();
//         self.outline(context);
//         context.fill();
//     }
// }

// struct WidgetContainer<T> where T: Widget {
//     transform: Matrix3x2,
//     widget: T,
// }

// struct List<T> where T: Widget {
//     list: Vec<T>,
// }
