
use std::rc::{Rc, Weak};

use prelude::*;

mod frame;
mod empty;
mod layout;
mod button;
mod label;
mod vbox;
mod hbox;
mod drawingarea;
mod marginbox;
mod centeringbox;
mod scrollbar;

pub use self::empty::Empty;
pub use self::layout::Layout;
pub use self::button::Button;
pub use self::label::Label;
pub use self::frame::Frame;
pub use self::vbox::VBox;
pub use self::hbox::HBox;
pub use self::drawingarea::DrawingArea;
pub use self::marginbox::MarginBox;
pub use self::centeringbox::CenteringBox;
pub use self::scrollbar::HScrollBar;

macro_rules! impl_anywidget {
    ( $($i:ident),* ) => {
        #[derive(Clone)]
        pub enum AnyWidget {
            $(
                $i(Rc<$i>),
            )*
        }

        impl AnyWidget {
            /// Create a weak reference
            pub fn weak_ref(&self) -> AnyWidgetWeakRef {
                match *self {
                    $(
                        AnyWidget::$i(ref w) => {
                            AnyWidgetWeakRef::$i(Rc::downgrade(w))
                        },
                    )*
                }
            }
        }

        /// AnyWidget can call methods of WidgetTrait directly.
        impl WidgetTrait for AnyWidget {
            fn render(&self, renderer: &mut Renderer) -> Result<(), String> {
                match *self { $( AnyWidget::$i(ref w) => {
                    return w.render(renderer);
                }, )* }
            }
            fn alloc_rect(&self, rect: Rect) {
                match *self { $( AnyWidget::$i(ref w) => {
                    w.alloc_rect(rect);
                }, )* }
            }
            fn event_handler(&self, event: &Event) {
                match *self { $( AnyWidget::$i(ref w) => {
                    w.event_handler(event);
                }, )* }
            }
            fn requested_size(&self) -> Option<(u32, u32)> {
                match *self { $( AnyWidget::$i(ref w) => {
                    return w.requested_size();
                }, )* }
            }
        }

        impl PartialEq for AnyWidget {
            fn eq(&self, other: &AnyWidget) -> bool {
                match *self { $( AnyWidget::$i(ref w) => {
                    if let AnyWidget::$i(ref o) = *other {
                        let p1: *const _ = &**w;
                        let p2: *const _ = &**o;
                        return p1 == p2;
                    }
                    return false;
                }, )* }
            }
        }

        impl Eq for AnyWidget {}

        $(
            impl From<Rc<$i>> for AnyWidget {
                #[inline]
                fn from(w: Rc<$i>) -> AnyWidget {
                    AnyWidget::$i(w)
                }
            }
        )*

        pub enum AnyWidgetWeakRef {
            $(
                $i(Weak<$i>),
            )*
        }

        impl AnyWidgetWeakRef {
            /// Try upgrade to a AnyWidget.
            pub fn upgrade(&self) -> Option<AnyWidget> {
                match *self { $( AnyWidgetWeakRef::$i(ref w) => {
                    if let Some(a) = w.upgrade() {
                        return Some(AnyWidget::$i(a));
                    }else{
                        return None;
                    }
                }, )* }
            }
        }
        
    }
}

impl_anywidget!(
    Frame,
    Layout, VBox, HBox,
    MarginBox, CenteringBox,
    Button, Empty, Label, DrawingArea,
    HScrollBar);

