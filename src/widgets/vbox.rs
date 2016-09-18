use std::rc::Rc;
use std::cell::{RefCell, Cell};

use prelude::*;

pub struct VBox {
    rect: Cell<Option<Rect>>,
    max_width: Cell<u32>, // Max width of child widgets
    children: RefCell<Vec<(AnyWidget, bool)>>,
    box_updated: Cell<bool>,  // Positions of children is updated or not
}

impl VBox {
    pub fn new() -> Rc<VBox> {
        Rc::new(VBox{
            rect: Cell::new(None),
            max_width: Cell::new(0),
            children: RefCell::new(Vec::new()),
            box_updated: Cell::new(false),
        })
    }

    pub fn pack<W: Into<AnyWidget>>(&self, w: W, expand: bool) {
        let w = w.into();

        self.children.borrow_mut().push((w, expand));
        self.box_updated.set(false);
    }

    fn position_update(&self) {
        if self.rect.get().is_none() {
            return;
        }
        
        let box_rect = self.rect.get().unwrap();
        let box_width = box_rect.width();
        let box_height = box_rect.height();
        let box_x = box_rect.x();
        let box_y = box_rect.y();

        let mut sum_requested_height = 0;
        let mut max_width = 0;
        let mut n_expand_widget = 0;
        
        for &(ref c, expand) in self.children.borrow().iter() {
            if let Some((w, h)) = c.requested_size() {
                sum_requested_height += h;
                if w > max_width { max_width = w }
            }
            if expand { n_expand_widget += 1 }
        }
        
        let expand_widget_height = if box_height < sum_requested_height || n_expand_widget == 0 {
            0
        }else{
            (box_height - sum_requested_height) / n_expand_widget
        };

        let mut y = box_y;
        for &(ref c, expand) in self.children.borrow().iter() {
            let requested_height = if let Some((_, h)) = c.requested_size() { h }else{ 0 };
            let mut height = requested_height;
            if expand { height += expand_widget_height }
            c.alloc_rect(Rect::from((box_x, y, box_width, height)));
            y += height as i32;
        }
        
        self.box_updated.set(true);
        self.max_width.set(max_width);
    }
}

impl WidgetTrait for VBox {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        if !self.box_updated.get() {
            self.position_update();
        }
        
        let before_clip = renderer.clip_rect();
        renderer.set_clip_rect(self.rect.get());

        for &(ref c, _) in self.children.borrow().iter() {
            try!(c.render(renderer));
        }

        renderer.set_clip_rect(before_clip);
        
        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));
        self.box_updated.set(false);
    }

    fn event_handler(&self, event: &Event) {
        self.with_children(|w| {
            w.event_handler(event);
        });
    }

    fn requested_size(&self) -> Option<(u32, u32)> {
        let mut max_width = 0;
        self.with_children(|widget| {
            if let Some((w, _)) = widget.requested_size() {
                if max_width < w { max_width = w; }
            }
        });

        self.max_width.set(max_width);
        Some((1, max_width))
    }
}

impl ContainerTrait for VBox {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;

        for &(ref c, _) in self.children.borrow().iter() {
            f(&c);
        }
    }
}
