use std::rc::Rc;
use std::cell::{RefCell, Cell};

use prelude::*;

/// The child will be centered on the given rectangler.
pub struct CenteringBox {
    rect: Cell<Option<Rect>>,
    child: RefCell<Option<AnyWidget>>,
}

impl CenteringBox {
    pub fn new<W: Into<AnyWidget>>(w: W) -> Rc<CenteringBox> {
        Rc::new(CenteringBox{
            rect: Cell::new(None),
            child: RefCell::new(Some(w.into())),
        })
    }
    
    pub fn empty() -> Rc<CenteringBox> {
        Rc::new(CenteringBox{
            rect: Cell::new(None),
            child: RefCell::new(None),
        })
    }
}

impl WidgetTrait for CenteringBox {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        let child = self.child.borrow();
        if let Some(ref c) = *child {
            try!(c.render(renderer));
        }

        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));

        if let Some(ref c) = *self.child.borrow() {
            if let Some(size) = c.requested_size() {
                let center = rect.center();
                c.alloc_rect(Rect::from_center(center, size.0, size.1));
            }else{
                // Expand the child widget if requested_size returns None.
                c.alloc_rect(rect);
            }
        }
    }

    fn event_handler(&self, event: &Event) {
        self.with_child(|w| {
            w.event_handler(event);
        });
    }
}

impl ContainerTrait for CenteringBox {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;

        let child = self.child.borrow();

        if let Some(ref c) = *child {
            f(c);
        }
    }
}

impl OneChildContainerTrait for CenteringBox {
    fn with_child<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&AnyWidget) -> R {
        let child = self.child.borrow();

        if let Some(ref c) = *child {
            return Some(f(c));
        }else{
            return None;
        }
    }
}
