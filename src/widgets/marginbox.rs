use std::rc::Rc;
use std::cell::{RefCell, Cell};

use prelude::*;

pub struct MarginBox {
    rect: Cell<Option<Rect>>,
    child: RefCell<Option<AnyWidget>>,
    top: Cell<u32>,
    bottom: Cell<u32>,
    left: Cell<u32>,
    right: Cell<u32>,
}

impl MarginBox {
    pub fn new<W: Into<AnyWidget>>(
        w: W, top: u32, bottom: u32, left: u32, right: u32) -> Rc<MarginBox> {
        Rc::new(MarginBox{
            rect: Cell::new(None),
            child: RefCell::new(Some(w.into())),
            top: Cell::new(top),
            bottom: Cell::new(bottom),
            left: Cell::new(left),
            right: Cell::new(right),
        })
    }

    pub fn empty(top: u32, bottom: u32, left: u32, right: u32) -> Rc<MarginBox> {
        Rc::new(MarginBox{
            rect: Cell::new(None),
            child: RefCell::new(None),
            top: Cell::new(top),
            bottom: Cell::new(bottom),
            left: Cell::new(left),
            right: Cell::new(right),
        })
    }
    
    pub fn set_child<W: Into<AnyWidget>>(&self, w: W) {
        let w = w.into();
        *self.child.borrow_mut() = Some(w);
    }
}

impl WidgetTrait for MarginBox {
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
            // Margins of child widget
            let mut left = self.left.get();
            let mut right = self.right.get();
            let mut top = self.top.get();
            let mut bottom = self.bottom.get();

            // If given rect is smaller than margin
            if rect.width() < left + right {
                left = 0;
                right = 0;
            }
            if rect.height() < top + bottom {
                top = 0;
                bottom = 0;
            }
            
            c.alloc_rect(Rect::new(
                rect.x() + left as i32,
                rect.y() + top as i32,
                rect.width() - left - right,
                rect.height() - top - bottom,
            ));
        }
    }

    fn event_handler(&self, event: &Event) {
        self.with_child(|w| {
            w.event_handler(event);
        });
    }

    fn requested_size(&self) -> Option<(u32, u32)> {
        let child = self.child.borrow();

        if let Some(ref c) = *child {
            let size = c.requested_size();
            if let Some(size) = size {
                Some((size.0 + self.left.get() + self.right.get(), size.1 + self.top.get() + self.bottom.get()))
            }else{
                Some((self.left.get() + self.right.get(), self.top.get() + self.bottom.get()))
            }
        }else{
            Some((self.left.get() + self.right.get(), self.top.get() + self.bottom.get()))
        }
    }
}

impl ContainerTrait for MarginBox {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;

        let child = self.child.borrow();

        if let Some(ref c) = *child {
            f(c);
        }
    }
}

impl OneChildContainerTrait for MarginBox {
    fn with_child<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&AnyWidget) -> R {
        let child = self.child.borrow();

        if let Some(ref c) = *child {
            return Some(f(c));
        }else{
            return None;
        }
    }
}
