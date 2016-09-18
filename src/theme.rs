
use std::rc::Rc;
use std::cell::RefCell;

use ::sdl2::pixels::Color;
use ::sdl2_ttf::Font;
use ::sdl2_ttf::Sdl2TtfContext;
//use ::sdl2_ttf::FontStyle;

pub struct ThemeLoader;

struct FontTheme {
    font_context: Option<Sdl2TtfContext>,
    default_font: Option<Rc<Font>>,
}

#[derive(Clone, Copy, Debug)]
pub struct ThemeColor {
    pub background      : Color,
    pub light_background: Color,
    pub dark_background : Color,
    pub light_border    : Color,
    pub dark_border     : Color,
    pub text            : Color,
}

impl ThemeColor {
    fn load() -> ThemeColor {
        ThemeColor::default()
    }
}

impl Default for ThemeColor {
    fn default() -> ThemeColor {
        ThemeColor{
            background      : Color::RGB(0xD4, 0xD4, 0xD4),
            light_background: Color::RGB(0xFF, 0xFF, 0xFF),
            dark_background : Color::RGB(0xC0, 0xC0, 0xC0),
            light_border    : Color::RGB(0xF0, 0xF0, 0xF0),
            dark_border     : Color::RGB(0x70, 0x70, 0x70),
            text            : Color::RGB(0x00, 0x00, 0x00),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ThemeLayout {
    pub size_scrollbar: u32,
}

impl ThemeLayout {
    fn load() -> ThemeLayout {
        ThemeLayout::default()
    }
}

impl Default for ThemeLayout {
    fn default() -> ThemeLayout {
        ThemeLayout{
            size_scrollbar: 20,
        }
    }
}

thread_local!(static FONT_THEME: RefCell<FontTheme> = RefCell::new(FontTheme{
    font_context: None,
    default_font: None,
}));

lazy_static! {
    static ref THEME_COLOR: ThemeColor = ThemeColor::load();
    static ref THEME_LAYOUT: ThemeLayout = ThemeLayout::load();
}

pub fn load_theme_default<T: AsRef<::std::path::Path>>(font_path: T)
                                                       -> Result<ThemeLoader, ThemeLoadError> {
    let f_context = ::sdl2_ttf::init().unwrap();
    let default_font = match f_context.load_font(font_path.as_ref(), 12) {
        Ok(f) => f,
        Err(e) => { return Err(ThemeLoadError::LoadFontError(e)) },
    };
    
    FONT_THEME.with(|c| {
        let mut c = c.borrow_mut();

        c.font_context = Some(f_context);
        c.default_font = Some(Rc::new(default_font));
    });
    Ok(ThemeLoader)
}

#[derive(Clone, Debug)]
pub enum ThemeLoadError {
    LoadFontError(String),
}

use std::fmt;
impl fmt::Display for ThemeLoadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ThemeLoadError::LoadFontError(ref s) => {
                write!(f, "Failed font loading \"{}\"", s)
            },
        }
    }
}

impl ::std::error::Error for ThemeLoadError {
    fn description(&self) -> &str {
        "theme initialization failed"
    }
}

pub fn get_theme_color() -> &'static ThemeColor {
    &*THEME_COLOR
}

pub fn get_theme_layout() -> &'static ThemeLayout {
    &*THEME_LAYOUT
}

pub fn get_background_color() -> Color {
    THEME_COLOR.background
}

pub fn get_light_border_color() -> Color {
    THEME_COLOR.light_border
}

pub fn get_dark_border_color() -> Color {
    THEME_COLOR.dark_border
}

pub fn get_text_color() -> Color {
    THEME_COLOR.text
}

pub fn get_default_font() -> Rc<Font> {
    FONT_THEME.with(|c| {
        let c = c.borrow();
        c.default_font.as_ref().expect("get_default_font").clone()
    })
}


