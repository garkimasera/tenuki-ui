use std::rc::Rc;
use std::cell::{RefCell, Cell};
use ::sdl2::mouse::Mouse;

use prelude::*;
use theme::*;
use enums::ButtonState;
use uicontext::request_redraw;

const PADDING_SIZE: u32 = 3;

pub struct Button {
    rect: Cell<Option<Rect>>,
    child: RefCell<Option<AnyWidget>>,
    callback_clicked: RefCell<Option<Box<Fn() + 'static>>>,
    state: Cell<ButtonState>,
}

impl Button {
    pub fn new() -> Rc<Button> {
        Rc::new(Button{
            rect: Cell::new(None),
            child: RefCell::new(None),
            callback_clicked: RefCell::new(None),
            state: Cell::new(ButtonState::None),
        })
    }

    pub fn new_with_label(s: &str) -> Rc<Button> {
        Rc::new(Button{
            rect: Cell::new(None),
            child: RefCell::new(Some(::widgets::Label::new(s).into())),
            callback_clicked: RefCell::new(None),
            state: Cell::new(ButtonState::None),
        })
    }
}

impl WidgetTrait for Button {
    fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
        let rect = self.rect.get().expect("No rectangle allocation at rendering");
        let color_theme = ::theme::get_theme_color();

        // Render lines
        renderer.set_draw_color(get_background_color());
        try!(renderer.fill_rect(rect));

        let (inside_border_color, outside_border_color, button_color)
            = if self.state.get() != ButtonState::Pressed {
                (color_theme.light_border, color_theme.dark_border, color_theme.background)
            }else{
                (color_theme.dark_border, color_theme.light_border, color_theme.dark_background)
            };
        
        
        renderer.set_draw_color(button_color);
        try!(renderer.fill_rect(rect));
        renderer.set_draw_color(outside_border_color);
        try!(renderer.draw_rect(rect));

        if rect.width() > 2 && rect.height() > 2 {
            renderer.set_draw_color(inside_border_color);
            try!(renderer.draw_rect(Rect::new(
                rect.x() + 1, rect.y() + 1, rect.width() - 2, rect.height() - 2)));
        }

        //renderer.set_draw_color(outside_border_color);
        // try!(renderer.draw_lines(&[
        //     Point::new(rect.x(), rect.y() + rect.height() as i32 - 1),
        //     Point::new(rect.x(), rect.y()),
        //     Point::new(rect.x() + rect.width() as i32 - 1, rect.y()),
        // ]));
        //renderer.set_draw_color(inside_border_color);
        //try!(renderer.draw_lines(&[
        //    Point::new(rect.x(), rect.y() + rect.height() as i32 - 1),
        //    Point::new(rect.x() + rect.width() as i32 - 1, rect.y() + rect.height() as i32 - 1),
        //    Point::new(rect.x() + rect.width() as i32 - 1, rect.y()),
        //]));

        // Render child widget
        if let Some(ref c) = *self.child.borrow() {
            try!(c.render(renderer));
        }
        
        Ok(())
    }

    fn alloc_rect(&self, rect: Rect) {
        self.rect.set(Some(rect));

        if let Some(ref c) = *self.child.borrow() {
            // Margins of child widget
            let mut left = PADDING_SIZE;
            let mut right = PADDING_SIZE;
            let mut top = PADDING_SIZE;
            let mut bottom = PADDING_SIZE;

            // If given rect is smaller than padding
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
        match *event {
            Event::ButtonDown((x, y), mouse_btn) => {
                // If this event is an click event to this button
                if mouse_btn == Mouse::Left && self.rect.get().unwrap().contains((x, y)) {
                    self.state.set(ButtonState::Pressed);
                    
                    request_redraw();
                }
            },
            Event::ButtonUp((_, _), mouse_btn) => {
                let state = self.state.get();
                if mouse_btn == Mouse::Left {
                    self.state.set(ButtonState::None);
                    if state == ButtonState::Pressed {
                        if let Some(ref c) = *self.callback_clicked.borrow() {
                            c();
                        }
                        request_redraw();
                    }
                }
            },
            Event::WindowFocusLost => {
                if self.state.get() != ButtonState::None {
                    self.state.set(ButtonState::None);
                    request_redraw();
                }
            },
            _ => (),
        }
    }

    fn requested_size(&self) -> Option<(u32, u32)> {
        let child = self.child.borrow();
        
        Some(if let Some(ref c) = *child {
            if let Some(size) = c.requested_size() {
                (size.0 + PADDING_SIZE * 2, size.1 + PADDING_SIZE * 2)
            }else{
                (PADDING_SIZE * 2, PADDING_SIZE * 2)
            }
        }else{
            (PADDING_SIZE * 2, PADDING_SIZE * 2)
        })
    }
}

impl ButtonTrait for Button {
    fn on_clicked<F>(&self, f: F) where F: Fn() + 'static {
        *self.callback_clicked.borrow_mut() = Some(Box::new(f));
    }
}

impl ContainerTrait for Button {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;

        let child = self.child.borrow();

        if let Some(ref c) = *child {
            f(c);
        }
    }
}

impl OneChildContainerTrait for Button {
    fn with_child<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&AnyWidget) -> R {
        let child = self.child.borrow();

        if let Some(ref c) = *child {
            return Some(f(c));
        }else{
            return None;
        }
    }
}

