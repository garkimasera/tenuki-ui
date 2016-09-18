
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use ::sdl2::mouse::Mouse;

use prelude::*;
use theme;
use uicontext::request_redraw;

fn get_bar_size() -> u32 {
    theme::get_theme_layout().size_scrollbar
}

macro_rules! impl_range {
    ($w:ty) => {
        impl RangeTrait for $w {
            fn set_lower(&self, lower: f64) {
                self.lower.set(lower);
            }
            fn get_lower(&self) -> f64 {
                self.lower.get()
            }
            fn set_upper(&self, upper: f64) {
                self.upper.set(upper);
            }
            fn get_upper(&self) -> f64 {
                self.upper.get()
            }
            fn set_value(&self, value: f64) {
                let value = if value < self.lower.get() {
                    self.lower.get()
                }else if value > self.upper.get() {
                    self.upper.get()
                }else{
                    value
                };
                    
                self.value.set(value);
                if let Some(ref f) = *self.callback_value_changed.borrow() {
                    f(value);
                }
            }
            fn get_value(&self) -> f64 {
                self.value.get()
            }
            fn step_dec_value(&self) {
                self.set_value(self.value.get() - self.step.get());
            }
            fn step_inc_value(&self) {
                self.set_value(self.value.get() + self.step.get());
            }
            fn on_value_changed<F>(&self, f: F) where F: Fn(f64) + 'static {
                *self.callback_value_changed.borrow_mut() = Some(Box::new(f));
            }
        }
    }
}

pub struct HScrollBar {
    rect: Cell<Option<Rect>>,
    left_arrow_box: Cell<Option<Rect>>,
    right_arrow_box: Cell<Option<Rect>>,

    // Members for RangeTrait
    lower: Cell<f64>,
    upper: Cell<f64>,
    value: Cell<f64>,
    step: Cell<f64>,
    callback_value_changed: RefCell<Option<Box<Fn(f64) + 'static>>>,
}

impl HScrollBar {
    pub fn new() -> Rc<HScrollBar> {
        Rc::new(HScrollBar{
            rect: Cell::new(None),
            left_arrow_box: Cell::new(None),
            right_arrow_box: Cell::new(None),
            lower: Cell::new(0.0), upper: Cell::new(1.0), value: Cell::new(0.0), step: Cell::new(0.1),
            callback_value_changed: RefCell::new(None),
        })
    }
}

impl WidgetTrait for HScrollBar {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        let rect = self.rect.get().expect("No rectangle allocation at rendering");
        let left_arrow_box = self.left_arrow_box.get().unwrap();
        let right_arrow_box = self.right_arrow_box.get().unwrap();
        let color = theme::get_theme_color();

        
        let bar_size = get_bar_size();

        let top_left_corner = (rect.x(), rect.y());
        renderer.set_draw_color(color.dark_background);
        try!(renderer.fill_rect(Rect::new(
            top_left_corner.0, top_left_corner.1, rect.width(), bar_size)));
        renderer.set_draw_color(color.dark_border);
        try!(renderer.draw_rect(Rect::new(
            top_left_corner.0, top_left_corner.1, rect.width(), bar_size)));
        
        renderer.set_draw_color(color.background);
        try!(renderer.fill_rect(left_arrow_box));
        renderer.set_draw_color(color.dark_border);
        try!(renderer.draw_rect(left_arrow_box));

        renderer.set_draw_color(color.background);
        try!(renderer.fill_rect(right_arrow_box));
        renderer.set_draw_color(color.dark_border);
        try!(renderer.draw_rect(right_arrow_box));
        
        
        
        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));

        let bar_size = get_bar_size();
        let top_left_corner = (rect.x(), rect.y());
        let top_right_corner = (rect.x() + rect.width() as i32, rect.y());
        
        self.left_arrow_box.set(Some(Rect::new(
            top_left_corner.0, top_left_corner.1 + 1, bar_size - 2, bar_size - 2)));
        self.right_arrow_box.set(Some(Rect::new(
            top_right_corner.0 - bar_size as i32 + 2, top_right_corner.1 + 1, bar_size - 2, bar_size - 2)));
    }

    fn event_handler(&self, event: &Event) {
        let rect_left_arrow_box = self.left_arrow_box.get().unwrap();
        let rect_right_arrow_box = self.right_arrow_box.get().unwrap();
        
        match *event {
            Event::ButtonDown((x, y), mouse_btn) => {
                if mouse_btn == Mouse::Left && self.rect.get().unwrap().contains((x, y)) {
                    if rect_left_arrow_box.contains((x, y)) { // Decreases value
                        self.step_dec_value();
                        request_redraw();
                    }else if rect_right_arrow_box.contains((x, y)) { // Increases value
                        self.step_inc_value();
                        request_redraw();
                    }
                }
            },
            _ => {},
        }
    }

    fn requested_size(&self) -> Option<(u32, u32)> {
        Some((get_bar_size() * 2, get_bar_size()))
    }
}

impl_range!(HScrollBar);
