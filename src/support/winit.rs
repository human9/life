//! A function for converting a `glium::glutin::Event` to a `conrod::event::Input`.

extern crate glium;

use conrod::position::Scalar;
use conrod::event::Input;
use conrod::input;
use conrod::input::keyboard;
use conrod::input::Key;
use conrod::cursor;


/// Types that have access to a `glium::glutin::Window` and can provide the necessary dimensions and hidpi
/// factor for converting `glium::glutin::Event`s to `conrod::event::Input`, as well as set the mouse
/// cursor.
///
/// This allows users to pass either `glium::Display`, `glium::glutin::Window` or `glium::glutin::Window`
/// to the `conrod::backend::glium::glutin::convert` function defined below.
pub trait WinitWindow {
    /// Return the inner size of the window.
    fn get_inner_size(&self) -> Option<(u32, u32)>;
    /// Return the window's DPI factor so that we can convert from pixel values to scalar values.
    fn hidpi_factor(&self) -> f32;
}

impl WinitWindow for glium::glutin::Window {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        glium::glutin::Window::get_inner_size(self)
    }
    fn hidpi_factor(&self) -> f32 {
        glium::glutin::Window::hidpi_factor(self)
    }
}

impl WinitWindow for glium::Display {
    fn get_inner_size(&self) -> Option<(u32, u32)> {
        self.gl_window().get_inner_size()
    }
    fn hidpi_factor(&self) -> f32 {
        self.gl_window().hidpi_factor()
    }
}


/// A function for converting a `glium::glutin::Event` to a `conrod::event::Input`.
///
/// This can be useful for single-window applications.
pub fn convert_event<W>(e: glium::glutin::Event, window: &W) -> Option<Input>
    where W: WinitWindow,
{
    match e {
        glium::glutin::Event::WindowEvent { event, .. } => convert_window_event(event, window),
        _ => None,
    }
}

/// A function for converting a `glium::glutin::WindowEvent` to a `conrod::event::Input`.
///
/// This is useful for multi-window applications.
pub fn convert_window_event<W>(e: glium::glutin::WindowEvent, window: &W) -> Option<Input>
    where W: WinitWindow,
{
    // The window size in points.
    let (win_w, win_h) = match window.get_inner_size() {
        Some((w, h)) => (w as Scalar, h as Scalar),
        None => return None,
    };

    // The "dots per inch" factor. Multiplying this by `win_w` and `win_h` gives the framebuffer
    // width and height.
    let dpi_factor = window.hidpi_factor() as Scalar;

    // Translate the coordinates from top-left-origin-with-y-down to centre-origin-with-y-up.
    //
    // winit produces input events in pixels, so these positions need to be divided by the widht
    // and height of the window in order to be DPI agnostic.
    let tx = |x: Scalar| (x / dpi_factor) - win_w / 2.0;
    let ty = |y: Scalar| -((y / dpi_factor) - win_h / 2.0);

    match e {

        glium::glutin::WindowEvent::Resized(w, h) => {
            let w = (w as Scalar / dpi_factor) as u32;
            let h = (h as Scalar / dpi_factor) as u32;
            Some(Input::Resize(w, h).into())
        },

        glium::glutin::WindowEvent::ReceivedCharacter(ch) => {
            let string = match ch {
                // Ignore control characters and return ascii for Text event (like sdl2).
                '\u{7f}' | // Delete
                '\u{1b}' | // Escape
                '\u{8}'  | // Backspace
                '\r' | '\n' | '\t' => "".to_string(),
                _ => ch.to_string()
            };
            Some(Input::Text(string).into())
        },

        glium::glutin::WindowEvent::Focused(focused) =>
            Some(Input::Focus(focused).into()),

        glium::glutin::WindowEvent::KeyboardInput { input, .. } => {
            input.virtual_keycode.map(|key| {
                match input.state {
                    glium::glutin::ElementState::Pressed =>
                        Input::Press(input::Button::Keyboard(map_key(key))).into(),
                    glium::glutin::ElementState::Released =>
                        Input::Release(input::Button::Keyboard(map_key(key))).into(),
                }
            })
        },

        glium::glutin::WindowEvent::Touch(glium::glutin::Touch { phase, location: (x, y), id, .. }) => {
            let phase = match phase {
                glium::glutin::TouchPhase::Started => input::touch::Phase::Start,
                glium::glutin::TouchPhase::Moved => input::touch::Phase::Move,
                glium::glutin::TouchPhase::Cancelled => input::touch::Phase::Cancel,
                glium::glutin::TouchPhase::Ended => input::touch::Phase::End,
            };
            let xy = [tx(x), ty(y)];
            let id = input::touch::Id::new(id);
            let touch = input::Touch { phase: phase, id: id, xy: xy };
            Some(Input::Touch(touch).into())
        }

        glium::glutin::WindowEvent::CursorMoved { position: (x, y), .. } => {
            let x = tx(x as Scalar);
            let y = ty(y as Scalar);
            let motion = input::Motion::MouseCursor { x: x, y: y };
            Some(Input::Motion(motion).into())
        },

        glium::glutin::WindowEvent::MouseWheel { delta, .. } => match delta {

            glium::glutin::MouseScrollDelta::PixelDelta(x, y) => {
                let x = x as Scalar / dpi_factor;
                let y = -y as Scalar / dpi_factor;
                let motion = input::Motion::Scroll { x: x, y: y };
                Some(Input::Motion(motion).into())
            },

            glium::glutin::MouseScrollDelta::LineDelta(x, y) => {
                // This should be configurable (we should provide a LineDelta event to allow for this).
                const ARBITRARY_POINTS_PER_LINE_FACTOR: Scalar = 10.0;
                let x = ARBITRARY_POINTS_PER_LINE_FACTOR * x as Scalar;
                let y = ARBITRARY_POINTS_PER_LINE_FACTOR * -y as Scalar;
                Some(Input::Motion(input::Motion::Scroll { x: x, y: y }).into())
            },
        },

        glium::glutin::WindowEvent::MouseInput { state, button, .. } => match state {
            glium::glutin::ElementState::Pressed =>
                Some(Input::Press(input::Button::Mouse(map_mouse(button))).into()),
            glium::glutin::ElementState::Released =>
                Some(Input::Release(input::Button::Mouse(map_mouse(button))).into()),
        },

        glium::glutin::WindowEvent::Refresh => {
            Some(Input::Redraw)
        },

        _ => None,
    }
}

/// Maps winit's key to a conrod `Key`.
pub fn map_key(keycode: glium::glutin::VirtualKeyCode) -> keyboard::Key {
    use self::keyboard::Key;

    match keycode {
        glium::glutin::VirtualKeyCode::Key0 => Key::D0,
        glium::glutin::VirtualKeyCode::Key1 => Key::D1,
        glium::glutin::VirtualKeyCode::Key2 => Key::D2,
        glium::glutin::VirtualKeyCode::Key3 => Key::D3,
        glium::glutin::VirtualKeyCode::Key4 => Key::D4,
        glium::glutin::VirtualKeyCode::Key5 => Key::D5,
        glium::glutin::VirtualKeyCode::Key6 => Key::D6,
        glium::glutin::VirtualKeyCode::Key7 => Key::D7,
        glium::glutin::VirtualKeyCode::Key8 => Key::D8,
        glium::glutin::VirtualKeyCode::Key9 => Key::D9,
        glium::glutin::VirtualKeyCode::A => Key::A,
        glium::glutin::VirtualKeyCode::B => Key::B,
        glium::glutin::VirtualKeyCode::C => Key::C,
        glium::glutin::VirtualKeyCode::D => Key::D,
        glium::glutin::VirtualKeyCode::E => Key::E,
        glium::glutin::VirtualKeyCode::F => Key::F,
        glium::glutin::VirtualKeyCode::G => Key::G,
        glium::glutin::VirtualKeyCode::H => Key::H,
        glium::glutin::VirtualKeyCode::I => Key::I,
        glium::glutin::VirtualKeyCode::J => Key::J,
        glium::glutin::VirtualKeyCode::K => Key::K,
        glium::glutin::VirtualKeyCode::L => Key::L,
        glium::glutin::VirtualKeyCode::M => Key::M,
        glium::glutin::VirtualKeyCode::N => Key::N,
        glium::glutin::VirtualKeyCode::O => Key::O,
        glium::glutin::VirtualKeyCode::P => Key::P,
        glium::glutin::VirtualKeyCode::Q => Key::Q,
        glium::glutin::VirtualKeyCode::R => Key::R,
        glium::glutin::VirtualKeyCode::S => Key::S,
        glium::glutin::VirtualKeyCode::T => Key::T,
        glium::glutin::VirtualKeyCode::U => Key::U,
        glium::glutin::VirtualKeyCode::V => Key::V,
        glium::glutin::VirtualKeyCode::W => Key::W,
        glium::glutin::VirtualKeyCode::X => Key::X,
        glium::glutin::VirtualKeyCode::Y => Key::Y,
        glium::glutin::VirtualKeyCode::Z => Key::Z,
        glium::glutin::VirtualKeyCode::Apostrophe => Key::Unknown,
        glium::glutin::VirtualKeyCode::Backslash => Key::Backslash,
        glium::glutin::VirtualKeyCode::Back => Key::Backspace,
        // K::CapsLock => Key::CapsLock,
        glium::glutin::VirtualKeyCode::Delete => Key::Delete,
        glium::glutin::VirtualKeyCode::Comma => Key::Comma,
        glium::glutin::VirtualKeyCode::Down => Key::Down,
        glium::glutin::VirtualKeyCode::End => Key::End,
        glium::glutin::VirtualKeyCode::Return => Key::Return,
        glium::glutin::VirtualKeyCode::Equals => Key::Equals,
        glium::glutin::VirtualKeyCode::Escape => Key::Escape,
        glium::glutin::VirtualKeyCode::F1 => Key::F1,
        glium::glutin::VirtualKeyCode::F2 => Key::F2,
        glium::glutin::VirtualKeyCode::F3 => Key::F3,
        glium::glutin::VirtualKeyCode::F4 => Key::F4,
        glium::glutin::VirtualKeyCode::F5 => Key::F5,
        glium::glutin::VirtualKeyCode::F6 => Key::F6,
        glium::glutin::VirtualKeyCode::F7 => Key::F7,
        glium::glutin::VirtualKeyCode::F8 => Key::F8,
        glium::glutin::VirtualKeyCode::F9 => Key::F9,
        glium::glutin::VirtualKeyCode::F10 => Key::F10,
        glium::glutin::VirtualKeyCode::F11 => Key::F11,
        glium::glutin::VirtualKeyCode::F12 => Key::F12,
        glium::glutin::VirtualKeyCode::F13 => Key::F13,
        glium::glutin::VirtualKeyCode::F14 => Key::F14,
        glium::glutin::VirtualKeyCode::F15 => Key::F15,
        // K::F16 => Key::F16,
        // K::F17 => Key::F17,
        // K::F18 => Key::F18,
        // K::F19 => Key::F19,
        // K::F20 => Key::F20,
        // K::F21 => Key::F21,
        // K::F22 => Key::F22,
        // K::F23 => Key::F23,
        // K::F24 => Key::F24,
        // Possibly next code.
        // K::F25 => Key::Unknown,
        glium::glutin::VirtualKeyCode::Numpad0 => Key::NumPad0,
        glium::glutin::VirtualKeyCode::Numpad1 => Key::NumPad1,
        glium::glutin::VirtualKeyCode::Numpad2 => Key::NumPad2,
        glium::glutin::VirtualKeyCode::Numpad3 => Key::NumPad3,
        glium::glutin::VirtualKeyCode::Numpad4 => Key::NumPad4,
        glium::glutin::VirtualKeyCode::Numpad5 => Key::NumPad5,
        glium::glutin::VirtualKeyCode::Numpad6 => Key::NumPad6,
        glium::glutin::VirtualKeyCode::Numpad7 => Key::NumPad7,
        glium::glutin::VirtualKeyCode::Numpad8 => Key::NumPad8,
        glium::glutin::VirtualKeyCode::Numpad9 => Key::NumPad9,
        glium::glutin::VirtualKeyCode::NumpadComma => Key::NumPadDecimal,
        glium::glutin::VirtualKeyCode::Divide => Key::NumPadDivide,
        glium::glutin::VirtualKeyCode::Multiply => Key::NumPadMultiply,
        glium::glutin::VirtualKeyCode::Subtract => Key::NumPadMinus,
        glium::glutin::VirtualKeyCode::Add => Key::NumPadPlus,
        glium::glutin::VirtualKeyCode::NumpadEnter => Key::NumPadEnter,
        glium::glutin::VirtualKeyCode::NumpadEquals => Key::NumPadEquals,
        glium::glutin::VirtualKeyCode::LShift => Key::LShift,
        glium::glutin::VirtualKeyCode::LControl => Key::LCtrl,
        glium::glutin::VirtualKeyCode::LAlt => Key::LAlt,
        glium::glutin::VirtualKeyCode::LMenu => Key::LGui,
        glium::glutin::VirtualKeyCode::RShift => Key::RShift,
        glium::glutin::VirtualKeyCode::RControl => Key::RCtrl,
        glium::glutin::VirtualKeyCode::RAlt => Key::RAlt,
        glium::glutin::VirtualKeyCode::RMenu => Key::RGui,
        // Map to backslash?
        // K::GraveAccent => Key::Unknown,
        glium::glutin::VirtualKeyCode::Home => Key::Home,
        glium::glutin::VirtualKeyCode::Insert => Key::Insert,
        glium::glutin::VirtualKeyCode::Left => Key::Left,
        glium::glutin::VirtualKeyCode::LBracket => Key::LeftBracket,
        // K::Menu => Key::Menu,
        glium::glutin::VirtualKeyCode::Minus => Key::Minus,
        glium::glutin::VirtualKeyCode::Numlock => Key::NumLockClear,
        glium::glutin::VirtualKeyCode::PageDown => Key::PageDown,
        glium::glutin::VirtualKeyCode::PageUp => Key::PageUp,
        glium::glutin::VirtualKeyCode::Pause => Key::Pause,
        glium::glutin::VirtualKeyCode::Period => Key::Period,
        // K::PrintScreen => Key::PrintScreen,
        glium::glutin::VirtualKeyCode::Right => Key::Right,
        glium::glutin::VirtualKeyCode::RBracket => Key::RightBracket,
        // K::ScrollLock => Key::ScrollLock,
        glium::glutin::VirtualKeyCode::Semicolon => Key::Semicolon,
        glium::glutin::VirtualKeyCode::Slash => Key::Slash,
        glium::glutin::VirtualKeyCode::Space => Key::Space,
        glium::glutin::VirtualKeyCode::Tab => Key::Tab,
        glium::glutin::VirtualKeyCode::Up => Key::Up,
        // K::World1 => Key::Unknown,
        // K::World2 => Key::Unknown,
        _ => Key::Unknown,
    }
}

/// Maps winit's mouse button to conrod's mouse button.
pub fn map_mouse(mouse_button: glium::glutin::MouseButton) -> input::MouseButton {
    use self::input::MouseButton;
    match mouse_button {
        glium::glutin::MouseButton::Left => MouseButton::Left,
        glium::glutin::MouseButton::Right => MouseButton::Right,
        glium::glutin::MouseButton::Middle => MouseButton::Middle,
        glium::glutin::MouseButton::Other(0) => MouseButton::X1,
        glium::glutin::MouseButton::Other(1) => MouseButton::X2,
        glium::glutin::MouseButton::Other(2) => MouseButton::Button6,
        glium::glutin::MouseButton::Other(3) => MouseButton::Button7,
        glium::glutin::MouseButton::Other(4) => MouseButton::Button8,
        _ => MouseButton::Unknown
    }
}

/// Convert a given conrod mouse cursor to the corresponding winit cursor type.
pub fn convert_mouse_cursor(cursor: cursor::MouseCursor) -> glium::glutin::MouseCursor {
    match cursor {
        cursor::MouseCursor::Text => glium::glutin::MouseCursor::Text,
        cursor::MouseCursor::VerticalText => glium::glutin::MouseCursor::VerticalText,
        cursor::MouseCursor::Hand => glium::glutin::MouseCursor::Hand,
        cursor::MouseCursor::Grab => glium::glutin::MouseCursor::Grab,
        cursor::MouseCursor::Grabbing => glium::glutin::MouseCursor::Grabbing,
        cursor::MouseCursor::ResizeVertical => glium::glutin::MouseCursor::NsResize,
        cursor::MouseCursor::ResizeHorizontal => glium::glutin::MouseCursor::EwResize,
        cursor::MouseCursor::ResizeTopLeftBottomRight => glium::glutin::MouseCursor::NwseResize,
        cursor::MouseCursor::ResizeTopRightBottomLeft => glium::glutin::MouseCursor::NeswResize,
        _ => glium::glutin::MouseCursor::Arrow,
    }
}
