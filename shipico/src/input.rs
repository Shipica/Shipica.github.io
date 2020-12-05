use once_cell::sync::Lazy;
use web_sys::{KeyboardEvent, MouseEvent, /* Performance, */ WheelEvent};

use crate::{
    math::{Point, Vec2},
    ui::ui,
};

#[derive(Debug, Clone)]
pub enum InputMouseEvent {
    Click(Point),

    StartDrag(Point, Vec2),
    Drag(Point, Vec2),
    EndDrag(Point),

    Wheel(f64),

    None,
}

#[derive(Debug, Clone)]
pub struct InputEvent {
    pub mouse_event: InputMouseEvent,
    pub keys_lately: Keys,
    pub keys: Keys,
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct Keys: u64 {
        const DELETE =      0b0000_0000_0000_0001;
        const MENU =        0b0000_0000_0000_0010;
        const ARROW_LEFT =  0b0000_0000_0000_0100;
        const ARROW_RIGHT = 0b0000_0000_0000_1000;
        const ARROW_UP =    0b0000_0000_0001_0000;
        const ARROW_DOWN =  0b0000_0000_0010_0000;
        const SHIFT =       0b0000_0000_0100_0000;
        const CTRL =        0b0000_0000_1000_0000;
        const ALT =         0b0000_0001_0000_0000;
    }
}

pub struct InternalInput {
    // perf: Performance,
    dragging_lately: bool,
    dragging_now: bool,

    mouse_down_lately: bool,
    mouse_down: bool,

    // mouse_down_time: f64,
    mouse_down_pos: Point,
    mouse_pos: Point,

    mouse_delta_current: Vec2,
    mouse_delta_till_mouse_down: Vec2,

    wheel_delta: f64,

    keys_lately: Keys,
    keys: Keys,
}

impl Default for InternalInput {
    fn default() -> Self {
        InternalInput {
            // perf: web_sys::window().unwrap().performance().unwrap(),
            dragging_lately: Default::default(),
            dragging_now: Default::default(),
            mouse_down_lately: Default::default(),
            mouse_down: Default::default(),
            // mouse_down_time: Default::default(),
            mouse_down_pos: Default::default(),
            mouse_delta_current: Default::default(),
            mouse_delta_till_mouse_down: Default::default(),
            keys_lately: Default::default(),
            keys: Default::default(),
            wheel_delta: Default::default(),
            mouse_pos: Default::default(),
        }
    }
}

static mut _INPUT: Lazy<*mut InternalInput> =
    Lazy::new(|| Box::leak(Box::new(InternalInput::default())));
pub struct Input;
const INPUT: Input = Input;

unsafe impl Send for Input {}
unsafe impl Sync for Input {}

pub const fn input() -> Input {
    INPUT
}

impl std::ops::Deref for Input {
    type Target = InternalInput;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { (*_INPUT).as_ref().unwrap() }
    }
}

impl std::ops::DerefMut for Input {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { (*_INPUT).as_mut().unwrap() }
    }
}

// TODO: change inputs to higly specified values
// to allow testing.
impl InternalInput {
    const DRAG_DELTA_THRESHOLD: f64 = 500.0;
    // const CLICK_TIME_MS: f64 = 500.0;

    // fn now(&self) -> f64 {
    //     self.perf.now()
    // }

    fn key_code_from_str(&self, key_code: &str) -> Option<Keys> {
        match key_code {
            "KeyX" => Some(Keys::DELETE),
            "Space" => Some(Keys::MENU),
            "Shift" => Some(Keys::SHIFT),
            "Ctrl" => Some(Keys::CTRL),
            "Alt" => Some(Keys::ALT),
            "ArrowDown" => Some(Keys::ARROW_DOWN),
            "ArrowLeft" => Some(Keys::ARROW_LEFT),
            "ArrowUp" => Some(Keys::ARROW_UP),
            "ArrowRight" => Some(Keys::ARROW_RIGHT),
            _ => None,
        }
    }

    fn update(&mut self) {
        self.keys_lately = self.keys;
        self.mouse_down_lately = self.mouse_down;
        self.dragging_lately = self.dragging_now;
        self.wheel_delta = 0.0;
    }

    fn update_ui(&self) {
        ui().update(InputEvent {
            mouse_event: self.resolve_mouse(),
            keys_lately: self.keys_lately,
            keys: self.keys,
        })
    }

    pub fn on_mouse_down(&mut self, event: MouseEvent) {
        self.update();
        self.mouse_down = true;
        self.mouse_down_pos = (event.x() as f64, event.y() as f64).into();
        self.mouse_pos = self.mouse_down_pos;
        // self.mouse_down_time = self.now();
        self.mouse_delta_till_mouse_down = [0.0, 0.0].into();
        self.update_ui();
    }

    pub fn on_mouse_up(&mut self, event: MouseEvent) {
        self.update();
        self.mouse_down = false;
        self.dragging_now = false;
        self.mouse_pos = (event.x() as f64, event.y() as f64).into();
        self.update_ui();
    }

    pub fn on_mouse_move(&mut self, event: MouseEvent) {
        self.update();
        let new_pos = (event.x() as f64, event.y() as f64).into();
        self.mouse_delta_current = new_pos - self.mouse_pos;
        self.mouse_pos = new_pos;

        if self.mouse_down {
            self.mouse_delta_till_mouse_down =
                self.mouse_delta_till_mouse_down + self.mouse_delta_current;

            if self.mouse_delta_till_mouse_down.len_squared() > Self::DRAG_DELTA_THRESHOLD {
                self.dragging_now = true;
            }
        } else {
            self.dragging_now = false;
        }
        self.update_ui();
    }

    pub fn on_wheel(&mut self, wheel: WheelEvent) {
        self.update();
        self.wheel_delta = wheel.delta_y();
        self.update_ui();
    }

    pub fn resolve_mouse(&self) -> InputMouseEvent {
        if self.wheel_delta != 0.0 {
            return InputMouseEvent::Wheel(self.wheel_delta);
        }

        if self.dragging_lately && self.dragging_now {
            return InputMouseEvent::Drag(self.mouse_pos, self.mouse_delta_current);
        }

        if !self.dragging_lately && self.dragging_now {
            return InputMouseEvent::StartDrag(
                self.mouse_down_pos,
                self.mouse_delta_till_mouse_down,
            );
        }

        if self.dragging_lately && !self.dragging_now {
            return InputMouseEvent::EndDrag(
                self.mouse_down_pos + self.mouse_delta_till_mouse_down,
            );
        }

        if !self.mouse_down
            && self.mouse_down_lately
            && self.mouse_delta_till_mouse_down.len_squared() <= Self::DRAG_DELTA_THRESHOLD
        // && self.now() - self.mouse_down_time <= Self::CLICK_TIME_MS
        {
            return InputMouseEvent::Click(self.mouse_down_pos);
        }
        InputMouseEvent::None
    }

    pub fn on_key_up(&mut self, event: KeyboardEvent) {
        self.update();
        if let Some(key_code) = self.key_code_from_str(&event.code()) {
            self.keys.remove(key_code);
        }
        self.update_ui();
    }

    pub fn on_key_down(&mut self, event: KeyboardEvent) {
        self.update();
        if let Some(key_code) = self.key_code_from_str(&event.code()) {
            self.keys.insert(key_code);
        }
        self.update_ui();
    }
}
