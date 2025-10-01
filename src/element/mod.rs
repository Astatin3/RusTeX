use std::rc::Rc;

mod rasterizer;
mod text_parser;
mod element_parser;
mod functions;
mod symbols;

pub enum KElement {
    LinearGroup(Vec<KElement>),
    Integer(i64),
    Decimal(f64),
    Text(String),
    Fraction {
        upper: Rc<KElement>, 
        lower: Rc<KElement>
    },
    SuperSub{
        inner: Rc<KElement>, 
        upper: Option<Rc<KElement>>,
        lower: Option<Rc<KElement>>
    },

    Plus,
    Minus,
    Equals,
    PlusMinus,
}