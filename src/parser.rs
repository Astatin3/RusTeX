use std::rc::Rc;

use fontdue::layout::{Layout, TextStyle};

use crate::{bitmap::Bitmap, fonts::FONTS};

pub enum KElement {
    LinearGroup(Vec<KElement>),
    Integer(i64),
    Decimal(f64),
    Fraction(Rc<KElement>, Rc<KElement>),
}

pub static FRACTION_SCALE: f32 = 1.;
// pub static FRACTION_PADDING: usize = 10;

// pub enum KSymbol {
//     Text {
//         data: String,
//         x: f32,
//         y: f32,
//         scale: f32
//     },
//     None,
// }

impl KElement {
    pub fn rasterize(&self, layout: &mut Layout, scale: f32) -> Bitmap {
        match self {
            KElement::LinearGroup(elems) => {
                let (mut totalx, mut maxy) = (0,0);
                let mut positions = Vec::new();
                for elem in elems {
                    let (x,y) = elem.get_bounds(layout, scale);
                    positions.push((totalx,y));
                    totalx += x;
                    maxy = maxy.max(y);
                }

                let mut bitmap = Bitmap::new(totalx, maxy);

                for i in 0..elems.len() {
                    let elem = &elems[i];
                    let pos = positions[i];
                    let new_bitmap = elem.rasterize(layout, scale);
                    // println!("{:?} {:?} {:?}", (bitmap.width, bitmap.height), pos, (new_bitmap.width, new_bitmap.height));
                    bitmap.overlay(&new_bitmap, pos.0, (maxy-pos.1)/2);
                }

                bitmap
            }
            KElement::Integer(i) => {
                render_text_block(layout, &i.to_string(), scale)
            },
            KElement::Decimal(i) => {
                render_text_block(layout, &i.to_string(), scale)
            },
            KElement::Fraction(a,b) => {
                let (ax,ay) = a.get_bounds(layout, scale * FRACTION_SCALE);
                let (bx,by) = b.get_bounds(layout, scale * FRACTION_SCALE);
                let (width, height) = (ax.max(bx), ay+by);

                let mut bitmap = Bitmap::new(width, height);
                
                let bitmap_a = &mut a.rasterize(layout, scale * FRACTION_SCALE);
                let bitmap_b = &mut b.rasterize(layout, scale * FRACTION_SCALE);

                if bitmap_a.width > bitmap_b.width {
                    bitmap.overlay(&bitmap_a, 0, 0);
                    bitmap.overlay(&bitmap_b, (bitmap_a.width-bitmap_b.width)/2, ay);
                } else {
                    bitmap.overlay(&bitmap_a, (bitmap_b.width-bitmap_a.width)/2, 0);
                    bitmap.overlay(&bitmap_b, 0, ay);
                }

                bitmap

                // symbols.append();
                // symbols.append(&mut b.to_text_style(x, midy, scale / 2.));
                // symbols
            
            }
        }
    }
    pub fn get_bounds(&self, layout: &mut Layout, scale: f32) -> (usize, usize) {
        match self {
            KElement::LinearGroup(elems) => {
                let (mut totalx, mut maxy) = (0,0);
                for elem in elems {
                    let (x,y) = elem.get_bounds(layout, scale);
                    totalx += x;
                    maxy = maxy.max(y);
                }
                (totalx, maxy)
            }
            KElement::Integer(i) => {
                measure_text_bounds(layout, &i.to_string(), scale)
            },
            KElement::Decimal(i) => {
                measure_text_bounds(layout, &i.to_string(), scale)
            },
            KElement::Fraction(a,b) => {
                let (ax,ay) = a.get_bounds(layout, scale * FRACTION_SCALE);
                let (bx,by) = b.get_bounds(layout, scale * FRACTION_SCALE);
                (ax.max(bx), ay+by)
            },

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
//     pub fn rasterize(&self, layout: &mut Layout, bitmap: &mut Bitmap) {
//         match self {
//             KSymbol::Text { data, x, y, scale } => {
//                 let new_bitmap = render_text_block(layout, data, *scale);
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