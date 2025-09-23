use std::rc::Rc;

use crate::element::{text_parser::ParsedObject, KElement};

fn assert_args(n: usize, start: usize, end: usize, err: &str) -> Result<(), String> {
    if start <= n && n <= end {
        Ok(())
    } else {
        Err(format!("{err}, {n} != [{start}, {end}]"))
    }
}


impl KElement {
    pub fn from_function(name: &str, args: &Vec<Vec<ParsedObject>>) -> Result<KElement, String> {
        match name {
            "frac" => {
                assert_args(args.len(), 2, 2, "A fraction must have 2 arguments!")?;

                Ok(KElement::Fraction { 
                    upper: Rc::new(Self::parse_object(&args[0])?), 
                    lower: Rc::new(Self::parse_object(&args[1])?) 
                })
            }
            "pm" => {
                assert_args(args.len(), 0, 0, "Symbol cannot take in any args!")?;

                Ok(KElement::Text("±".to_string()))
            }
            _ =>  Err(format!("Invalid function: \\{}", name))
        }
    }
}