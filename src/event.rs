
use prelude::*;
use ::sdl2::mouse::Mouse;

/// Events are processed by widgets which window owns.
#[derive(Clone)]
pub enum Event<'a> {
    ButtonDown((i32, i32), Mouse),
    ButtonUp((i32, i32), Mouse),
    RendererChanged(&'a Renderer<'a>),
    WindowFocusLost,
}
