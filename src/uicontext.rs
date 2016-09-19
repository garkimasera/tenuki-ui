
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};
use std::thread::sleep;

use ::sdl2;
use ::sdl2::{Sdl, VideoSubsystem};
use ::sdl2::event::Event as SdlEvent;
use ::sdl2::event::WindowEventId;
use ::sdl2::keyboard::Keycode;
use ::sdl2::video::Window as SdlWindow;

use prelude::*;
use ::widgets::Frame;
use ::theme::ThemeLoader;

struct UIContext {
    pub sdl_context: Sdl,
    pub video_subsystem: VideoSubsystem,
    frames: RefCell<Vec<Rc<Frame>>>,
    prev_instant: Instant
}

impl UIContext {
    pub fn new() -> Result<UIContext, String> {
        let sdl_context = try!(sdl2::init());
        let video_subsystem = try!(sdl_context.video());

        Ok(UIContext{
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            frames: RefCell::new(Vec::new()),
            prev_instant: Instant::now(),
        })
    }

    /// Start main loop
    pub fn main_loop(&self) {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let fps_duration = Duration::from_millis(1000 / 20);
        let frames = self.frames.borrow();
        let mut prev_instant: Instant = self.prev_instant;

        let send_event = |event: Event| {
            frames[0].event_handler(&event);
        };

        request_redraw();
        
        'running: loop {
            for sdl_event in event_pump.poll_iter() {
                match sdl_event {
                    SdlEvent::Quit {..}  => {
                        break 'running
                    },
                    SdlEvent::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        
                    },
                    SdlEvent::MouseButtonDown {x, y, mouse_btn, .. } => {
                        send_event(Event::ButtonDown((x, y), mouse_btn));
                    },
                    SdlEvent::MouseButtonUp {x, y, mouse_btn, .. } => {
                        send_event(Event::ButtonUp((x, y), mouse_btn));
                    },
                    SdlEvent::Window {win_event_id, .. } => {
                        match win_event_id {
                            WindowEventId::FocusLost => {
                                send_event(Event::WindowFocusLost);
                            },
                            WindowEventId::SizeChanged => {
                                request_redraw();
                                frames[0].size_update();
                            }
                            _ => {},
                        }
                    }
                    _ => {}
                }
                
            }

            if redraw_start() {
                for frame in frames.iter() {
                    frame.update();
                }
            }
            
            let new_instant = Instant::now();
            if new_instant > prev_instant + fps_duration {
                // Skip next drawing
            }else{
                let used_time = new_instant.duration_since(prev_instant);
                sleep(fps_duration - used_time);
            }
            prev_instant = Instant::now();
        }
    }

    /// Add new window
    pub fn add_frame(&self, frame: Rc<Frame>) {
        let mut frames = self.frames.borrow_mut();
        frames.push(frame);
    }
}

thread_local!(static UI_CONTEXT: RefCell<Option<UIContext>> = RefCell::new(None));

pub fn init(theme_loader: ThemeLoader) -> Result<(), String> {
    let _ = theme_loader;
    
    UI_CONTEXT.with(|uicontext| {
        if uicontext.borrow().is_some() {
            panic!("Double initialization of uicontext");
        }
        match UIContext::new() {
            Ok(result) => { *uicontext.borrow_mut() = Some(result); Ok(()) },
            Err(e) => { Err(e) },
        }
    })
}

pub fn create_window(
    title: &str, width: u32, height: u32, resizable: bool)
                     -> Result<SdlWindow, ::sdl2::video::WindowBuildError> {
    UI_CONTEXT.with(|uicontext| {
        match *uicontext.borrow() {
            Some(ref uicontext) => {
                let mut window_builder = uicontext.video_subsystem.window(title, width, height);
                if resizable { window_builder.resizable(); }
                
                window_builder.build()
            },
            None => { panic!("Creating frame before uicontext initialization"); },
        }
    })
}

/// Start main loop
pub fn main_loop() {
    UI_CONTEXT.with(|uicontext| {
        match *uicontext.borrow() {
            Some(ref uicontext) => {
                uicontext.main_loop();
            },
            None => { panic!("Enter main loop before uicontext initialization"); },
        }
    });
}

pub fn add_frame(frame: Rc<Frame>) {
    UI_CONTEXT.with(|uicontext| {
        match *uicontext.borrow() {
            Some(ref uicontext) => {
                uicontext.add_frame(frame);
            },
            None => { panic!("Enter main loop before uicontext initialization"); },
        }
    });
}

// If redrawing is needed, set true
thread_local!(static REDRAW_REQUESTED: Cell<bool> = Cell::new(false));

pub fn request_redraw() {
    REDRAW_REQUESTED.with(|a| a.set(true));
}

fn redraw_start() -> bool {
    REDRAW_REQUESTED.with(|a| {
        let is_requested = a.get();
        a.set(false);
        is_requested
    })
}

