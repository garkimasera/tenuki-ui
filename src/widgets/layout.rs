use std::rc::Rc;
use std::cell::{Cell, RefCell};

use prelude::*;
use ::theme::get_background_color;

pub struct Layout {
    rect: Cell<Option<Rect>>,
    children: RefCell<Vec<AnyWidget>>,
}

impl Layout {
    pub fn new() -> Rc<Layout> {
        Rc::new(Layout{
            rect: Cell::new(None),
            children: RefCell::new(Vec::new())
        })
    }

    pub fn add<T: Into<AnyWidget>>(&self, widget: T, rect: Rect) {
        let widget = widget.into();
        widget.alloc_rect(rect);
        self.children.borrow_mut().push(widget);
    }
}

impl WidgetTrait for Layout {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        renderer.set_draw_color(get_background_color());
        try!(renderer.fill_rect(self.rect.get().unwrap()));
        
        let n = self.children.borrow().len();

        for i in 0..n {
            let child = self.children.borrow()[i].clone();

            try!(child.render(renderer));
        }
        
        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));
    }

    fn event_handler(&self, event: &Event) {
        self.with_children(|w| {
            w.event_handler(event);
        });
    }
}

impl ContainerTrait for Layout {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;
        for w in self.children.borrow().iter() {
            f(w)
        }
    }
}
