use fontdue::layout::{Layout, TextStyle};

use crate::{bitmap::Bitmap, consts::*, element::KElement, fonts::FONTS, RusTeX};


impl KElement {
    pub fn rasterize(&self, globals: &mut RusTeX, current_scale: f32) -> Bitmap {
        match self {
            KElement::LinearGroup(elems) => {
                let (mut totalx, mut mintop, mut maxbottom): (usize, usize, usize) = (0,0,0);
                let mut positions = Vec::new();

                for elem in elems {
                    let (width, height, centery) = elem.get_bounds(globals, current_scale);
                    let top = centery;
                    let bottom = height - centery;

                    positions.push((totalx, centery));

                    mintop = mintop.max(top);
                    maxbottom = maxbottom.max(bottom);
                    totalx += width;
                }

                let height = maxbottom + mintop;


                let mut bitmap = Bitmap::new(totalx, height);

                for i in 0..elems.len() {
                    let elem = &elems[i];
                    let pos = positions[i];
                    let new_bitmap = elem.rasterize(globals, current_scale);
                    // println!("{:?} {:?} {:?}", (bitmap.width, bitmap.height), pos, (new_bitmap.width, new_bitmap.height));
                    // let center = (center - pos.2) / 2;
                    // let center = 0;
                    // println!("{}, {}, {}", center, 0, maxy);

                    let y = mintop - pos.1;

                    bitmap.overlay(&new_bitmap, pos.0, y);
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
            KElement::Fraction{upper,lower} => {
                let padding = (FRACTION_PADDING * current_scale) as usize;
                let (ax,ay, _) = upper.get_bounds(globals, current_scale * FRACTION_SCALE);
                let (bx,by, _) = lower.get_bounds(globals, current_scale * FRACTION_SCALE);

                let (width, height) = (
                    ax.max(bx) + padding*2, 
                    ay+by + padding
                );

                let mut bitmap = Bitmap::new(width, height);
                
                let bitmap_a = &mut upper.rasterize(globals, current_scale * FRACTION_SCALE);
                let bitmap_b = &mut lower.rasterize(globals, current_scale * FRACTION_SCALE);

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
            KElement::SuperSub{inner, upper, lower} => {
                let (ax, ay, _) = inner.get_bounds(globals, current_scale);
                if upper.is_some() && lower.is_some() {
                    todo!();
                } else if upper.is_some() {
                    let upper = upper.as_ref().unwrap();
                    let (bx, by, _) = upper.get_bounds(globals, current_scale * SUPERSCRIPT_SCALE);
                    let yoffset = (by as f32*SUPERSCRIPT_Y_OFFSET) as usize;

                    let (width, height) = (
                        ax+bx,
                        ay + yoffset
                    );
                    let mut bitmap = Bitmap::new(width, height);

                    bitmap.overlay(&inner.rasterize(globals, current_scale), 0, yoffset);
                    bitmap.overlay(&upper.rasterize(globals, current_scale * SUPERSCRIPT_SCALE), ax, 0);

                    bitmap

                } else if lower.is_some() {
                    todo!();
                } else {
                    unreachable!()
                }
            }
        }
    }
    pub fn get_bounds(&self, globals: &mut RusTeX, current_scale: f32) -> (usize, usize, usize) {
        match self {
            KElement::LinearGroup(elems) => {
                bounds_of_linear_group(elems, globals, current_scale)
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
            KElement::Fraction{upper,lower} => {
                let (ax,ay, _) = upper.get_bounds(globals, current_scale * FRACTION_SCALE);
                let (bx,by, _) = lower.get_bounds(globals, current_scale * FRACTION_SCALE);
                (
                    (ax.max(bx)) + 2*(FRACTION_PADDING * current_scale) as usize, 
                    ay+by + (FRACTION_PADDING * current_scale) as usize,
                    ay + (FRACTION_PADDING * current_scale) as usize,
                )
            },
            KElement::SuperSub{inner, upper, lower} => {
                let (ax, ay, center) = inner.get_bounds(globals, current_scale);
                if upper.is_some() && lower.is_some() {
                    todo!();
                } else if upper.is_some() {
                    let (bx, by, _) = upper.as_ref().unwrap().get_bounds(globals, current_scale * SUPERSCRIPT_SCALE);
                    (
                        ax+bx,
                        ay + (by as f32*SUPERSCRIPT_Y_OFFSET) as usize,
                        center + (by as f32*SUPERSCRIPT_Y_OFFSET) as usize
                    )
                } else if lower.is_some() {
                    todo!();
                } else {
                    unreachable!()
                }

            }

        }
    }
}

fn bounds_of_linear_group(elems: &Vec<KElement>, globals: &mut RusTeX, current_scale: f32) -> (usize, usize, usize) {
    // let common_centery = elems[0].get_bounds(globals, current_scale);

    let (mut totalx, mut mintop, mut maxbottom): (usize, usize, usize) = (0,0,0);
    for elem in elems {
        let (width, height, centery) = elem.get_bounds(globals, current_scale);
        totalx += width;
        let top = centery;
        let bottom = height - centery;


        mintop = mintop.max(top);
        maxbottom = maxbottom.max(bottom);
    }

    (
        totalx,
        maxbottom + mintop,
        mintop
    )
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

fn measure_text_bounds(layout: &mut Layout, text: &str, scale:f32) -> (usize, usize, usize) {
    layout.clear();
    layout.append(&FONTS, &TextStyle::new(text, scale, 0));

    let (mut width, mut height): (usize, usize) = (0,0);
    for glyph in layout.glyphs() {
        width = width.max(glyph.x as usize + glyph.width);
        height = height.max(glyph.y as usize + glyph.height);
    }

    (width, height, height/2)

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