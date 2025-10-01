use fontdue::Font;
use lazy_static::lazy_static;

pub static KaTeX_Main_Regular: &'static [u8] = include_bytes!("../fonts/KaTeX_Main-Regular.ttf");
pub static KaTeX_Main_Italic: &'static [u8] = include_bytes!("../fonts/KaTeX_Main-Italic.ttf");

lazy_static! {
    pub static ref FONTS: Vec<Font> = vec![
        Font::from_bytes(KaTeX_Main_Regular, fontdue::FontSettings::default()).unwrap(),
        Font::from_bytes(KaTeX_Main_Italic, fontdue::FontSettings::default()).unwrap(),
    ];
}