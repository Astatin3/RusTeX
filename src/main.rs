#[allow(non_upper_case_globals)] 

mod fonts;
mod element;
mod bitmap;
mod consts;

use std::{rc::Rc, time::Instant};

use fontdue::{layout::{CoordinateSystem, Layout, LayoutSettings}};

use crate::{bitmap::Bitmap, element::{KElement}};

fn main() -> Result<(), std::fmt::Error> {
    parse_test()
}

fn parse_test() -> Result<(), std::fmt::Error> {    
    // Test the quadratic formula
    // let tex_input = r"x=\frac{-b\pm\sqrt{b^2 - 4ac}}{2a}";
    let tex_input = &std::env::args().nth(1).unwrap();

    match KElement::parse(tex_input) {
        Ok(result) => {
            let mut rustex = RusTeX::new(TeXSettings { scale: 100. });
            rustex.rasterize(result).print();
        }
        Err(e) => println!("Error: {}", e),
    }

    // println!("Parsing: {}", tex_input);
    
    // match parser::parse(tex_input) {
    //     Ok(result) => {
    //         println!("Parsed result:");
    //         for (i, obj) in result.iter().enumerate() {
    //             println!("  [{}]: {:#?}", i, obj);
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error parsing TeX: {}", e);
    //     }
    // }
    
    // // Test some other examples
    // let examples = vec![
    //     "a^2",
    //     r"\frac{12.34a+b}{2}",
    //     r"\sqrt{x}",
    //     "x_{a^2}^{2^a}",
    //     "(a+b)^2",
    // ];
    
    // for example in examples {
    //     println!("\n--- Parsing: {} ---", example);
    //     match parser.parse(example) {
    //         Ok(result) => {
    //             for obj in result {
    //                 println!("{:#?}", obj);
    //             }
    //         }
    //         Err(e) => println!("Error: {}", e),
    //     }
    // }

    Ok(())
}

// fn raster_test() -> Result<(), std::fmt::Error> {
//     let start = Instant::now();

//     let mut rustex = RusTeX::new(TeXSettings { scale: 100. });

//     let element = 
    
//     KElement::Superscript(Rc::new(
//     KElement::LinearGroup(vec![
//         KElement::Fraction(
//             Rc::new(KElement::LinearGroup(vec![
//                 KElement::Integer(123),
//                 KElement::Text("*".to_string()),
//                 KElement::Superscript(
//                     Rc::new(KElement::Integer(123)),
//                     Rc::new(KElement::Fraction(
//                         Rc::new(KElement::Integer(5)),
//                         Rc::new(KElement::Integer(2)))
//                     ),
//                 )
//             ])),
//             Rc::new(KElement::Integer(12))
//         ),
//         // KElement::Integer(12),
//         // KElement::Integer(12),
//         KElement::Decimal(12.34),
//         KElement::Fraction(
//             Rc::new(KElement::Fraction(
//                 Rc::new(KElement::Integer(123)),
//                 Rc::new(KElement::Integer(12)))
//             ),
//             Rc::new(KElement::Fraction(
//                 Rc::new(KElement::Integer(123)),
//                 Rc::new(KElement::Fraction(
//                     Rc::new(KElement::Integer(123)),
//                     Rc::new(KElement::Fraction(
//                         Rc::new(KElement::Integer(123)),
//                         Rc::new(KElement::Fraction(
//                             Rc::new(KElement::Integer(123)),
//                             Rc::new(KElement::Fraction(
//                                 Rc::new(KElement::Integer(123)),
//                                 Rc::new(KElement::Fraction(
//                                     Rc::new(KElement::Integer(123)),
//                                     Rc::new(KElement::Fraction(
//                                         Rc::new(KElement::Integer(123)),
//                                         Rc::new(KElement::Decimal(1234.5678))
//                                     ))
//                                 ))
//                             ))
//                         ))
//                     ))
//                 ))
//             )),
//         ),
//     ])), Rc::new(KElement::Decimal(12.34)));

//     // rustex.add_text(&TextStyle::new("testi12345ng! e^23", 100.0, 0));
//     let bitmap = rustex.rasterize(element);

//     println!("Rasterizing time: {:?}", start.elapsed());
    
//     bitmap.print();

//     // print_bitmap(&bitmap, width, height);
    
//     Ok(())
// }

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