#[allow(non_upper_case_globals)] 

mod fonts;

use fontdue::{layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle}};
use icy_sixel::{
    DiffusionMethod, MethodForLargest, MethodForRep, PixelFormat, Quality, sixel_string,
};

fn main() -> Result<(), std::fmt::Error> {
    let mut rustex = RusTeX::new();
    rustex.add_text(&TextStyle::new("testi12345ng! e^23", 35.0, 0));
    let (bitmap, (width, height)) = rustex.rasterize();
    print_bitmap(&bitmap, width, height);
    Ok(())
}

struct RusTeX {
    layout: Layout,
}

impl RusTeX {
    pub fn new() -> Self {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        // By default, layout is initialized with the default layout settings. This call is redundant, but
        // demonstrates setting the value with your custom settings.
        layout.reset(&LayoutSettings {
            ..LayoutSettings::default()
        });

        Self {
            layout,
        }
    }

    pub fn add_text(&mut self, text_style: &TextStyle) {
        self.layout.append(&fonts::FONTS, text_style);
    }

    pub fn rasterize(&mut self) -> (Vec<u8>, (usize, usize)) {
        let (mut maxx, mut maxy): (usize, usize) = (0,0);
        for glyph in self.layout.glyphs() {
            maxx = maxx.max(glyph.x as usize + glyph.width);
            maxy = maxy.max(glyph.y as usize + glyph.height);
        }

        let mut bitmap: Vec<u8> = vec![0; maxx*maxy];
        
        for glyph in self.layout.glyphs() {

            let font = &fonts::FONTS[glyph.font_index];
            let (metrics, char_bitmap) = font.rasterize_config(glyph.key);

            
            
            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let pixel = char_bitmap[y*glyph.width + x];

                    // let index = (x+glyph.x as usize)*maxy + (y+glyph.y as usize);
                    let index = (y+glyph.y as usize)*maxx + (x+glyph.x as usize);

                    bitmap[index] = pixel;
                }
            }
        }

        (bitmap, (maxx, maxy))
    }
}

/// Prints a 1 byte per pixel greyscale bitmap to Sixel format in console
fn print_bitmap(bitmap: &Vec<u8>, width: usize, height: usize) {
    let mut bitmap_rgb888 = vec![0; width*height*3];


    for y in 0..height {
        for x in 0..width {
            let index = y*width + x;

            let pixel = bitmap[index];

            bitmap_rgb888[index*3] = pixel;
            bitmap_rgb888[index*3 + 1] = pixel;
            bitmap_rgb888[index*3 + 2] = pixel;
        }
    }
    



    let sixel_data = sixel_string(
        &bitmap_rgb888,
        width as i32,
        height as i32,
        PixelFormat::RGB888,
        DiffusionMethod::None,
        MethodForLargest::Auto,
        MethodForRep::Auto,
        Quality::AUTO,
    ).unwrap();

    println!("{}", sixel_data);

}