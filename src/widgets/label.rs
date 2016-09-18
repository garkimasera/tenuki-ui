use std::rc::Rc;
use std::cell::{RefCell, Cell};

use sdl2::surface::Surface;

use prelude::*;
use uicontext::request_redraw;

pub struct Label {
    label_str: RefCell<String>,
    rect: Cell<Option<Rect>>,
    text_size: Cell<(u32, u32)>,
}

impl Label {
    pub fn new(s: &str) -> Rc<Label> {
        let label = Label{
            label_str: RefCell::new(s.to_string()),
            rect: Cell::new(None),
            text_size: Cell::new((0, 0)),
        };
        label.recalc_textsize();
        Rc::new(label)
    }

    fn recalc_textsize(&self) {
        let font = ::theme::get_default_font();
        let text_size = font.size_of(&*self.label_str.borrow()).expect("Text size calculation");
        self.text_size.set(text_size);
    }
}

impl WidgetTrait for Label {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        let font = ::theme::get_default_font();
        let surface: Surface = font.render(&*self.label_str.borrow()).blended(
            ::theme::get_text_color()).expect("b");
        let texture = renderer.create_texture_from_surface(surface).expect("a");

        let (x, y, w, h) = self.rect.get().expect("Rect").into();
        let (text_w, text_h) = self.text_size.get();

        // Centering text if given rect is bigger than text size
        let x = if w > text_w { x + (w - text_w) as i32 / 2 }else{ x };
        let y = if h > text_h { y + (h - text_h) as i32 / 2 }else{ y };
        
        renderer.copy(&texture, None, Some(Rect::new(x, y, text_w, text_h)));

        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));
    }

    fn requested_size(&self) -> Option<(u32, u32)> {
        Some(self.text_size.get())
    }
}

impl TextTrait for Label {
    fn set_text(&self, new_text: &str) {
        *self.label_str.borrow_mut() = new_text.to_owned();
        self.recalc_textsize();
        request_redraw();
    }
    fn get_text(&self) -> String {
        self.label_str.borrow().clone()
    }
}
