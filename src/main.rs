#[allow(non_upper_case_globals)] 

mod fonts;
mod element;
mod bitmap;
mod consts;

use fontdue::{layout::{CoordinateSystem, Layout, LayoutSettings}};

use crate::{bitmap::Bitmap, element::{KElement}};

fn main() -> Result<(), std::fmt::Error> {
    parse_test()
}

fn parse_test() -> Result<(), std::fmt::Error> {    
    let tex_input = &std::env::args().nth(1).unwrap();

    match KElement::parse(tex_input) {
        Ok(result) => {
            let mut rustex = RusTeX::new(TeXSettings { scale: 100. });
            rustex.rasterize(result).print();
        }
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}


struct RusTeX {
    pub settings: TeXSettings,
    pub layout: Layout
}

struct TeXSettings {
    scale: f32,
}

impl RusTeX {
    pub fn new(settings: TeXSettings) -> Self {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);

        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });

        Self {
            settings,
            layout,
        }
    }

    // pub fn add_text_style(&mut self, text_style: &TextStyle) {
    //     // self.layout.append(&fonts::FONTS, text_style);
    // }

    pub fn rasterize(&mut self, root_element: KElement) -> Bitmap {
        let scale = self.settings.scale;
        root_element.rasterize(self, scale)
    }
}