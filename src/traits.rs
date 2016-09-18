
use ::prelude::*;

/// All widgets implement this trait
pub trait WidgetTrait {
    /// Render this widget on the given Renderer.
    fn render(&self, renderer: &mut Renderer) -> Result<(), String>;

    /// Sets a size and position on the screen.
    /// This is used by container widget.
    fn alloc_rect(&self, rect: Rect);

    /// Recieve and process event
    fn event_handler(&self, _event: &Event) {
    }

    /// Return adequate size for this widget
    fn requested_size(&self) -> Option<(u32, u32)> {
        None
    }
}

/// Widget which can contain other widgets 
pub trait ContainerTrait: WidgetTrait {
    fn with_children<F: FnMut(&AnyWidget)>(&self, f: F);
}

/// Container widget with one child
pub trait OneChildContainerTrait: ContainerTrait {
    fn with_child<F, R>(&self, f: F) -> Option<R> where F: FnOnce(&AnyWidget) -> R;
}

/// Button widgets
pub trait ButtonTrait: WidgetTrait {
    fn on_clicked<F>(&self, f: F) where F: Fn() + 'static;
}

/// Widgets that have text data
pub trait TextTrait: WidgetTrait {
    fn set_text(&self, new_text: &str);
    fn get_text(&self) -> String;
}

pub trait RangeTrait {
    fn set_lower(&self, lower: f64);
    fn get_lower(&self) -> f64;
    fn set_upper(&self, upper: f64);
    fn get_upper(&self) -> f64;
    fn set_value(&self, value: f64);
    fn get_value(&self) -> f64;
    fn step_dec_value(&self);
    fn step_inc_value(&self);
    fn on_value_changed<F>(&self, f: F) where F: Fn(f64) + 'static;
}

