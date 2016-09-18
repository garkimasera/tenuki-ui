use std::rc::Rc;
use std::cell::Cell;

use prelude::*;

pub struct Empty {
    rect: Cell<Option<Rect>>,
}

impl Empty {
    pub fn new() -> Rc<Empty> {
        Rc::new(Empty{
            rect: Cell::new(None),
        })
    }
}

impl WidgetTrait for Empty {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        renderer.set_draw_color(::sdl2::pixels::Color::RGB(0, 0, 0xFF)/*get_background_color()*/);
        try!(renderer.fill_rect(self.rect.get().unwrap()));

        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));
    }
}
