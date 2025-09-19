#[allow(non_upper_case_globals)] 

mod fonts;
mod parser;
mod bitmap;
mod consts;

use std::{rc::Rc, time::Instant};

use fontdue::{layout::{CoordinateSystem, Layout, LayoutSettings}};

use crate::{bitmap::Bitmap, parser::{KElement}};

fn main() -> Result<(), std::fmt::Error> {
    let start = Instant::now();

    let mut rustex = RusTeX::new(TeXSettings { scale: 100. });

    let element = KElement::LinearGroup(vec![
        KElement::Fraction(
            Rc::new(KElement::LinearGroup(vec![
                KElement::Integer(123),
                KElement::Text("*".to_string()),
                KElement::Superscript(
                    Rc::new(KElement::Integer(123)),
                    Rc::new(KElement::Integer(2))
                )
            ])),
            Rc::new(KElement::Integer(12))
        ),
        // KElement::Integer(12),
        // KElement::Integer(12),
        KElement::Decimal(12.34),
        KElement::Fraction(
            Rc::new(KElement::Fraction(
                Rc::new(KElement::Integer(123)),
                Rc::new(KElement::Integer(12)))
            ),
            Rc::new(KElement::Fraction(
                Rc::new(KElement::Integer(123)),
                Rc::new(KElement::Fraction(
                    Rc::new(KElement::Integer(123)),
                    Rc::new(KElement::Fraction(
                        Rc::new(KElement::Integer(123)),
                        Rc::new(KElement::Fraction(
                            Rc::new(KElement::Integer(123)),
                            Rc::new(KElement::Fraction(
                                Rc::new(KElement::Integer(123)),
                                Rc::new(KElement::Fraction(
                                    Rc::new(KElement::Integer(123)),
                                    Rc::new(KElement::Fraction(
                                        Rc::new(KElement::Integer(123)),
                                        Rc::new(KElement::Decimal(1234.5678))
                                    ))
                                ))
                            ))
                        ))
                    ))
                ))
            )),
        ),
    ]);

    // rustex.add_text(&TextStyle::new("testi12345ng! e^23", 100.0, 0));
    let bitmap = rustex.rasterize(element);

    println!("Rasterizing time: {:?}", start.elapsed());
    
    bitmap.print();

    // print_bitmap(&bitmap, width, height);
    
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