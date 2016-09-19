# tenuki-ui
Simple &amp; minimal widget toolkit written in Rust. It depends on SDL2 and its renderer.

## Example

```Rust
extern crate tenuki_ui as ui;

use ui::prelude::*;
use ui::uicontext;
use ui::widgets;
use ui::theme::load_theme_default;

fn main() {
    uicontext::init(load_theme_default("path_to_a_font_file.ttf").unwrap()).unwrap();
    
    let frame = widgets::Frame::new("tenuki-ui test", 300, 300, true);
    let vbox = widgets::VBox::new();
    let label = widgets::Label::new("Click Button!");
    vbox.pack(widgets::CenteringBox::new(label.clone()), true);
    
    let button = widgets::Button::new_with_label("Button");
    button.on_clicked(move || { label.set_text("Clicked!"); });
    vbox.pack(widgets::MarginBox::new(button, 10, 10, 10, 10), false);
    
    frame.set_child(vbox);

    uicontext::main_loop();
}
```

## License
MIT
