use std::rc::Rc;
use std::cell::{RefCell, Cell};

use prelude::*;

pub struct DrawingArea {
    rect: Cell<Option<Rect>>,
    callback_draw: RefCell<Option<Box<Fn(&mut Renderer) -> Result<(), String> + 'static>>>,
}

impl DrawingArea {
    pub fn new() -> Rc<DrawingArea> {
        Rc::new(DrawingArea{
            rect: Cell::new(None),
            callback_draw: RefCell::new(None),
        })
    }

    pub fn on_draw<F>(&self, f: F) where F: Fn(&mut Renderer) -> Result<(), String> + 'static {
        *self.callback_draw.borrow_mut() = Some(Box::new(f));
    }
}

impl WidgetTrait for DrawingArea {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        let callback_draw = self.callback_draw.borrow();
        let rect = self.rect.get().expect("No rectangle allocation at rendering");

        let before_clip = renderer.clip_rect();
        renderer.set_clip_rect(Some(rect));
        renderer.set_viewport(Some(rect));
        // Clear by white
        renderer.set_draw_color(Color::RGB(0xFF, 0xFF, 0xFF));
        renderer.clear();
        
        let mut result = Ok(());
        
        if let Some(ref callback) = *callback_draw {
            result = callback(renderer);
        }

        renderer.set_viewport(None);
        renderer.set_clip_rect(before_clip);
        result
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));
    }
}

