use std::rc::Rc;

use fontdue::layout::{Layout, TextStyle};

use crate::{bitmap::Bitmap, fonts::FONTS, RusTeX, consts::*};


pub enum KElement {
    LinearGroup(Vec<KElement>),
    Integer(i64),
    Decimal(f64),
    Text(String),
    Fraction(Rc<KElement>, Rc<KElement>),
    Superscript(Rc<KElement>, Rc<KElement>),
}

impl KElement {
    pub fn rasterize(&self, globals: &mut RusTeX, current_scale: f32) -> Bitmap {
        match self {
            KElement::LinearGroup(elems) => {
                let (mut totalx, mut maxy) = (0,0);
                let mut positions = Vec::new();
                for elem in elems {
                    let (x,y) = elem.get_bounds(globals, current_scale);
                    positions.push((totalx,y));
                    totalx += x;
                    maxy = maxy.max(y);
                }

                let mut bitmap = Bitmap::new(totalx, maxy);

                for i in 0..elems.len() {
                    let elem = &elems[i];
                    let pos = positions[i];
                    let new_bitmap = elem.rasterize(globals, current_scale);
                    // println!("{:?} {:?} {:?}", (bitmap.width, bitmap.height), pos, (new_bitmap.width, new_bitmap.height));
                    bitmap.overlay(&new_bitmap, pos.0, (maxy-pos.1)/2);
                }

                bitmap
            }
            KElement::Integer(i) => {
                render_text_block(&mut globals.layout, &i.to_string(), current_scale)
            },
            KElement::Decimal(i) => {
                render_text_block(&mut globals.layout, &i.to_string(), current_scale)
            },            
            KElement::Text(str) => {
                render_text_block(&mut globals.layout, &str, current_scale)
            },
            KElement::Fraction(a,b) => {
                let padding = (FRACTION_PADDING * globals.settings.scale) as usize;
                let (ax,ay) = a.get_bounds(globals, current_scale * FRACTION_SCALE);
                let (bx,by) = b.get_bounds(globals, current_scale * FRACTION_SCALE);

                let (width, height) = (
                    ax.max(bx) + padding*2, 
                    ay+by + padding
                );

                let mut bitmap = Bitmap::new(width, height);
                
                let bitmap_a = &mut a.rasterize(globals, current_scale * FRACTION_SCALE);
                let bitmap_b = &mut b.rasterize(globals, current_scale * FRACTION_SCALE);

                if bitmap_a.width > bitmap_b.width {
                    bitmap.overlay(&bitmap_a, padding, 0);
                    bitmap.overlay(&bitmap_b, padding+(bitmap_a.width-bitmap_b.width)/2, ay + padding);
                } else {
                    bitmap.overlay(&bitmap_a, padding+(bitmap_b.width-bitmap_a.width)/2, 0);
                    bitmap.overlay(&bitmap_b, padding, ay + padding);
                }

                bitmap.draw_line(0, ay+padding, bitmap.width, ay+padding, globals.settings.scale*LINE_WIDTH, 255);

                bitmap

                // symbols.append();
                // symbols.append(&mut b.to_text_style(x, midy, scale / 2.));
                // symbols
            
            }
            KElement::Superscript(a, b) => {
                let (ax, ay) = a.get_bounds(globals, current_scale);
                let (bx, by) = b.get_bounds(globals, current_scale * SUPERSCRIPT_SCALE);
                let yoffset = (by as f32*SUPERSCRIPT_Y_OFFSET) as usize;

                let (width, height) = (
                    ax+bx,
                    ay + yoffset
                );
                let mut bitmap = Bitmap::new(width, height);

                bitmap.overlay(&a.rasterize(globals, current_scale), 0, yoffset);
                bitmap.overlay(&b.rasterize(globals, current_scale * SUPERSCRIPT_SCALE), ax, 0);

                bitmap
            }
        }
    }
    pub fn get_bounds(&self, globals: &mut RusTeX, current_scale: f32) -> (usize, usize) {
        match self {
            KElement::LinearGroup(elems) => {
                let (mut totalx, mut maxy) = (0,0);
                for elem in elems {
                    let (x,y) = elem.get_bounds(globals, current_scale);
                    totalx += x;
                    maxy = maxy.max(y);
                }
                (totalx, maxy)
            }
            KElement::Integer(i) => {
                measure_text_bounds(&mut globals.layout, &i.to_string(), current_scale)
            },
            KElement::Decimal(i) => {
                measure_text_bounds(&mut globals.layout, &i.to_string(), current_scale)
            },
            KElement::Text(str) => {
                measure_text_bounds(&mut globals.layout, &str, current_scale)
            },
            KElement::Fraction(a,b) => {
                let (ax,ay) = a.get_bounds(globals, current_scale * FRACTION_SCALE);
                let (bx,by) = b.get_bounds(globals, current_scale * FRACTION_SCALE);
                (
                    (ax.max(bx)) + 2*(FRACTION_PADDING * globals.settings.scale) as usize, 
                    ay+by + (FRACTION_PADDING * globals.settings.scale) as usize
                )
            },
            KElement::Superscript(a, b) => {
                let (ax, ay) = a.get_bounds(globals, current_scale);
                let (bx, by) = b.get_bounds(globals, current_scale * SUPERSCRIPT_SCALE);
                (
                    ax+bx,
                    ay + (by as f32*SUPERSCRIPT_Y_OFFSET) as usize
                )
            }

        }
    }
}

// impl KSymbol {
//     // pub fn get_max_bounds(&self) -> (f32, f32) {
//     //     match self {
//     //         KSymbol::Text { x, y, ..} => (*x,*y),
//     //         _ => (0.,0.)
//     //     }
//     // }
//     pub fn rasterize(&self, globals.layout: &mut Layout, bitmap: &mut Bitmap) {
//         match self {
//             KSymbol::Text { data, x, y, scale } => {
//                 let new_bitmap = render_text_block(globals.layout, data, *scale);
//                 bitmap.overlay(&new_bitmap, *x as usize, *y as usize);
//             },
//             KSymbol::None => {},
//         }
//     }
// }

fn measure_text_bounds(layout: &mut Layout, text: &str, scale:f32) -> (usize, usize) {
    layout.clear();
    layout.append(&FONTS, &TextStyle::new(text, scale, 0));

    let (mut width, mut height): (usize, usize) = (0,0);
    for glyph in layout.glyphs() {
        width = width.max(glyph.x as usize + glyph.width);
        height = height.max(glyph.y as usize + glyph.height);
    }

    (width, height)

}

fn render_text_block(layout: &mut Layout, text: &str, scale:f32) -> Bitmap {
    layout.clear();

    layout.append(&FONTS, &TextStyle::new(text, scale, 0));

    let (mut width, mut height): (usize, usize) = (0,0);
    for glyph in layout.glyphs() {
        width = width.max(glyph.x as usize + glyph.width);
        height = height.max(glyph.y as usize + glyph.height);
    }

    let mut new_bitmap = Bitmap::new(width, height);

    for glyph in layout.glyphs() {

        let font = &FONTS[glyph.font_index];
        let (_, char_bitmap) = font.rasterize_config(glyph.key);
        
        new_bitmap.overlay(&Bitmap::from_data(char_bitmap, glyph.width, glyph.height), glyph.x as usize, glyph.y as usize);
    }

    new_bitmap
}