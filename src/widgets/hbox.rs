use std::rc::Rc;
use std::cell::{RefCell, Cell};

use prelude::*;

pub struct HBox {
    rect: Cell<Option<Rect>>,
    max_height: Cell<u32>, // Max height of child widgets
    children: RefCell<Vec<(AnyWidget, bool)>>,
    box_updated: Cell<bool>,  // Positions of children is updated or not
}

impl HBox {
    pub fn new() -> Rc<HBox> {
        Rc::new(HBox{
            rect: Cell::new(None),
            max_height: Cell::new(0),
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

        let mut sum_requested_width = 0;
        let mut max_height = 0;
        let mut n_expand_widget = 0;
        
        for &(ref c, expand) in self.children.borrow().iter() {
            if let Some((w, h)) = c.requested_size() {
                sum_requested_width += w;
                if h > max_height { max_height = h }
            }
            if expand { n_expand_widget += 1 }
        }

        let expand_widget_width = if box_width < sum_requested_width  || n_expand_widget == 0 {
            0
        }else{
            (box_width - sum_requested_width) / n_expand_widget
        };

        let mut x = box_x;
        for &(ref c, expand) in self.children.borrow().iter() {
            let requested_width = if let Some((w, _)) = c.requested_size() { w }else{ 0 };
            let mut width = requested_width;
            if expand { width += expand_widget_width }
            c.alloc_rect(Rect::from((x, box_y, width, box_height)));
            x += width as i32;
        }
        
        self.box_updated.set(true);
        self.max_height.set(max_height);
    }
}

impl WidgetTrait for HBox {
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
        let mut max_height = 0;
        self.with_children(|widget| {
            if let Some((_, h)) = widget.requested_size() {
                if max_height < h { max_height = h; }
            }
        });

        self.max_height.set(max_height);
        Some((1, max_height))
    }
}

impl ContainerTrait for HBox {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;

        for &(ref c, _) in self.children.borrow().iter() {
            f(&c);
        }
    }
}
