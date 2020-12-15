use once_cell::unsync::Lazy;

use crate::{
    canvas::Canvas,
    input::{InputEvent, InputMouseEvent, Keys},
    log,
    tree::Tree,
    widget::Component,
    widget::Widget,
    FloatingWindow, Settings,
};

pub struct InternalUi {
    pub tree: Tree,
    pub floating_window: FloatingWindow,
    pub canvas: Canvas,
    pub settings: Settings,
    _dirty: bool,
    _hooks: Hooks,
}

static mut _UI: Lazy<*mut InternalUi> = Lazy::new(|| Box::leak(Box::new(InternalUi::new())));
pub struct Ui;
const UI: Ui = Ui;

unsafe impl Send for Ui {}
unsafe impl Sync for Ui {}

pub const fn ui() -> Ui {
    UI
}

impl std::ops::Deref for Ui {
    type Target = InternalUi;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { (*_UI).as_ref().unwrap() }
    }
}

impl std::ops::DerefMut for Ui {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let mut ui = unsafe { (*_UI).as_mut().unwrap() };
        ui._dirty = true;
        ui
    }
}

impl InternalUi {
    fn new() -> InternalUi {
        let canvas = Canvas::new();
        let tree = Tree::new();
        let mut ui = InternalUi {
            tree,
            canvas,
            floating_window: Default::default(),
            settings: Default::default(),
            _hooks: Default::default(),
            _dirty: true,
        };
        ui.redraw();
        ui
    }

    pub fn update(&mut self, event: InputEvent) {
        let pressed = |keys: Keys| !event.keys_lately.contains(keys) && event.keys.contains(keys);
        let pressing = |keys: Keys| event.keys_lately.contains(keys) && event.keys.contains(keys);
        let released = |keys: Keys| event.keys_lately.contains(keys) && !event.keys.contains(keys);
        let no_keys = || event.keys.is_empty();
        let down = |keys: Keys| pressing(keys) || pressed(keys);
        // let not = |keys: Keys| !event.keys.contains(keys);

        match event.mouse_event {
            InputMouseEvent::Click(_) if no_keys() => {
                // Click only
                log!("click!");
            }
            InputMouseEvent::Click(_) if pressing(Keys::DELETE) => {
                log!("click delete!");
            }
            InputMouseEvent::StartDrag(pos, delta) if no_keys() => {
                self.tree.drag(delta);
            }
            InputMouseEvent::EndDrag(_) if no_keys() => {}
            InputMouseEvent::Drag(pos, delta) if no_keys() => {
                self.tree.drag(delta);
            }
            _ if pressed(Keys::MENU) => {
                log!("menu!");
            }
            _ if released(Keys::MENU) => {
                log!("menu released!");
            }
            _ if down(Keys::ARROW_DOWN)
                || down(Keys::ARROW_RIGHT)
                || down(Keys::ARROW_LEFT)
                || down(Keys::ARROW_UP) =>
            {
                const SPEED: f64 = 5.0;
                let mut x = 0.0;
                let mut y = 0.0;
                if down(Keys::ARROW_DOWN) {
                    y -= 1.0 * SPEED;
                }
                if down(Keys::ARROW_UP) {
                    y += 1.0 * SPEED;
                }
                if down(Keys::ARROW_RIGHT) {
                    x -= 1.0 * SPEED;
                }
                if down(Keys::ARROW_LEFT) {
                    x += 1.0 * SPEED;
                }
                self.tree.drag([x, y].into())
            }
            _ => {
                return;
            }
        }

        self.redraw();
    }

    pub fn redraw(&mut self) {
        log!("REDRAW!");

        self.canvas.reset();
        self.tree.build().draw(&mut self.canvas);
        self.draw_debug();

        // if let Some(phantom_connection) = &self.state.phantom_connection {
        //     phantom_connection.draw(&self.context);
        // }

        // if let Some(cut_line) = &self.state.cut_line {
        //     cut_line.draw(&self.context);
        // }

        // if self.state.space_button {
        //     self.floating_window.draw(&self.context);
        // }
    }

    fn draw_debug(&self) {
        let context = &self.canvas.render_context;
        let font_size = 30.0;
        context.set_font(&format!("{}px sans-serif", font_size));

        let mut last_y = 50.0;
        let mut fill_text = |text: String| {
            context.fill_text(&text, 50.0, last_y).unwrap();
            last_y += 40.0;
        };
        fill_text(format!("Zoom: {:1.3?}", self.tree.z()));
        fill_text(format!("X: {:4.3?}", self.tree.x()));
        fill_text(format!("Y: {:4.3?}", self.tree.y()));
        // fill_text(format!("Mouse down: {}", self.state.mouse_down));
        // fill_text(format!("Alt: {}", self.state.delete_button));
        // fill_text(format!(
        //     "Mouse x: {:4.3?}, y: {:4.3?}",
        //     self.state.mouse_pos.x, self.state.mouse_pos.y
        // ));
        // fill_text(format!(
        //     "Mouse delta: x: {:4.3?}, y: {:4.3?}",
        //     self.state.mouse_delta.x, self.state.mouse_delta.y
        // ));
        // let canvas_point = self.tree.screen_to_canvas(self.state.mouse_pos);
        // fill_text(format!(
        //     "Canvas mouse: x: {:4.3?}, y: {:4.3?}",
        //     canvas_point.x, canvas_point.y
        // ));
    }
}

macro_rules! hooks {
    ($($hook: ident($($param_type: tt),*)),*,) => {
        hooks!($($hook($($param_type),*)),*);
    };
    ($($hook: ident($($param_type: tt),*)),*) => {
        ::paste::paste! {
            use wasm_bindgen::{prelude::Closure, JsCast};
            use web_sys::*;

            #[derive(Default)]
            struct Hooks {
                $(
                    [<on_ $hook>]: Option<Closure<dyn FnMut($($param_type),*)>>
                ),*
            }

            $(
                pub fn [<set_on_$hook>]<F>(f: F)
                    where F: FnMut($($param_type),*) + 'static
                {
                    let closure = Closure::wrap(Box::new(f) as Box<dyn FnMut($($param_type),*)>);
                    ui().canvas.window.[<set_on $hook>](Some(closure.as_ref().unchecked_ref()));
                    ui()._hooks.[<on_ $hook>] = Some(closure);
                }
            )*
        }
    };
}

hooks!(
    resize(UiEvent),
    mousemove(MouseEvent),
    mousedown(MouseEvent),
    mouseup(MouseEvent),
    keydown(KeyboardEvent),
    keyup(KeyboardEvent),
    wheel(WheelEvent),
    contextmenu(MouseEvent),
);
