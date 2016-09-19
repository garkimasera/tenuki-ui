
use std::cell::{RefCell, Ref};
use std::rc::Rc;

use prelude::*;
use uicontext;
use ::sdl2::video::WindowRef;
//use ::sdl2::video::WindowBuilder;

/// Top level widget
pub struct Frame {
    renderer: RefCell<Renderer<'static>>,
    child: RefCell<Option<AnyWidget>>,
}

impl Frame {
    pub fn new(title: &str, width: u32, height: u32, resizable: bool) -> Rc<Frame> {
        
        let window = match uicontext::create_window(title, width, height, false) {
            Ok(w) => w,
            Err(e) => {
                println!("Frame creation failed.\n{}", e);
                panic!();
            },
        };
        let renderer = window.renderer().build().unwrap();

        let frame = Rc::new(Frame{
            renderer: RefCell::new(renderer),
            child: RefCell::new(None),
        });
        uicontext::add_frame(frame.clone());
        frame
    }

    /// Redraw this frame
    pub fn update(&self) {
        let mut renderer = self.renderer.borrow_mut();

        renderer.set_draw_color(::theme::get_background_color());
        renderer.clear();

        if let Some(ref mut child) = *self.child.borrow_mut() {
            let _ = child.render(&mut *renderer);
        }
        
        renderer.present();
    }

    /// Set a child
    pub fn set_child<W: Into<AnyWidget>>(&self, w: W) {
        let w = w.into();
        *self.child.borrow_mut() = Some(w);
        self.size_update();
    }

    // Update child widget's size
    pub fn size_update(&self) {
        if let Some(ref c) = *self.child.borrow() {
            let window_size = self.window().size();
            c.alloc_rect(Rect::new(0, 0, window_size.0, window_size.1));
        }
    }

    // Get window
    fn window(&self) -> Ref<WindowRef> {
        Ref::map(self.renderer.borrow(), |r| r.window().unwrap())
    }
}

impl WidgetTrait for Frame {
    fn render(&self, _renderer: &mut Renderer) -> Result<(), String> {
        unimplemented!();
    }
    
    fn alloc_rect(&self, _rect: Rect) {
        
    }

    fn event_handler(&self, event: &Event) {
        self.with_children(|w| {
            w.event_handler(event);
        });
    }
}

impl ContainerTrait for Frame {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F) {
        let mut f = f;
        if let Some(ref c) = *self.child.borrow() {
            f(c);
        }
    }
}
