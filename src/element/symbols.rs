use crate::{bitmap::Bitmap, consts::OPERATOR_X_PADDING, element::KElement, RusTeX};

pub const PLUS: &'static str = "+";
pub const MINUS: &'static str = "-";
pub const EQUALS: &'static str = "=";
pub const PLUS_MINUS: &'static str = "±";

pub const PLUS_YOFFSET: f32 = 0.15;
pub const MINUS_YOFFSET: f32 = 0.15;
pub const EQUALS_YOFFSET: f32 = 0.27;
pub const PLUSMMINUS_YOFFSET: f32 = 0.15;

impl KElement {
    pub fn from_symbol(symbol: &str) -> Result<KElement, String> {
        match symbol {
            "+" => Ok(KElement::Plus),
            "-" => Ok(KElement::Minus),
            "=" => Ok(KElement::Equals),
            "±" => Ok(KElement::PlusMinus),
            _ => unreachable!("Unimplemented symbol: {}", symbol),
            // _ => Ok(KElement::Text(symbol.to_string()))
        
            // MINUS => KElement::Minus
        }
    }

    pub fn rasterize_symbol(&self, globals: &mut RusTeX, current_scale: f32) -> Bitmap {
        match self {
            KElement::Plus => Self::render_text_block(&mut globals.layout, PLUS, 0, current_scale, OPERATOR_X_PADDING),
            KElement::Minus => Self::render_text_block(&mut globals.layout, MINUS, 0, current_scale, OPERATOR_X_PADDING),
            KElement::Equals => Self::render_text_block(&mut globals.layout, EQUALS, 0, current_scale, OPERATOR_X_PADDING),
            KElement::PlusMinus => Self::render_text_block(&mut globals.layout, PLUS_MINUS, 0, current_scale, OPERATOR_X_PADDING),
            _ => unreachable!()
        }
    }

    pub fn get_symbol_bounds(&self, globals: &mut RusTeX, current_scale: f32) -> (usize, usize, usize) {
        match self {
            KElement::Plus => Self::measure_text_bounds(&mut globals.layout, PLUS, 0, current_scale, OPERATOR_X_PADDING, PLUS_YOFFSET),
            KElement::Minus => Self::measure_text_bounds(&mut globals.layout, MINUS, 0, current_scale, OPERATOR_X_PADDING, MINUS_YOFFSET),
            KElement::Equals => Self::measure_text_bounds(&mut globals.layout, EQUALS, 0, current_scale, OPERATOR_X_PADDING, EQUALS_YOFFSET),
            KElement::PlusMinus => Self::measure_text_bounds(&mut globals.layout, PLUS_MINUS, 0, current_scale, OPERATOR_X_PADDING, PLUSMMINUS_YOFFSET),
            _ => unreachable!()
        }
    }
}