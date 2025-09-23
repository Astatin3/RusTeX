use std::rc::Rc;

use crate::element::{text_parser::{self, ParsedObject}, KElement};

impl KElement {
    pub fn parse(input: &str) -> Result<KElement, String> {
        let elems = text_parser::parse(input)?;
        Self::parse_object(&elems)
    }

    pub fn parse_object(elems: &Vec<ParsedObject>) -> Result<KElement, String> {

        let mut root = Vec::new();

        for elem in elems {
            match elem {
                text_parser::ParsedObject::Func { 
                    name, 
                    content, 
                    super_script, 
                    sub_script 
                } => {

                    if !super_script.is_empty() && !sub_script.is_empty() {
                        // root.push(KElement::SuperSub { inner: (), upper: (), lower: () }
                    } else if !super_script.is_empty() {
                        // root.push(KElement::SuperSub { inner: (), upper: (), lower: () }
                    } else if !sub_script.is_empty() {
                        // root.push(KElement::SuperSub { inner: (), upper: (), lower: () }
                    } else {
                        
                        root.push(Self::from_function(name, content)?);
                    }
                },
                text_parser::ParsedObject::Var { 
                    text, 
                    super_script, 
                    sub_script 
                } => {
                    if !super_script.is_empty() && !sub_script.is_empty() {
                        root.push(KElement::SuperSub { 
                            inner: Rc::new(KElement::Text(text.clone())), 
                            upper: Some(Rc::new(Self::parse_object(super_script)?)), 
                            lower: Some(Rc::new(Self::parse_object(sub_script)?)), 
                        });
                    } else if !super_script.is_empty() {
                        root.push(KElement::SuperSub { 
                            inner: Rc::new(KElement::Text(text.clone())), 
                            upper: Some(Rc::new(Self::parse_object(super_script)?)), 
                            lower: None 
                        });
                    } else if !sub_script.is_empty() {
                        root.push(KElement::SuperSub { 
                            inner: Rc::new(KElement::Text(text.clone())), 
                            upper: None,
                            lower: Some(Rc::new(Self::parse_object(super_script)?))
                        });
                    } else {
                        root.push(KElement::Text(text.clone()));
                    }
                },
                text_parser::ParsedObject::Operator { 
                    text 
                } => {
                    root.push(KElement::Text(text.clone()));
                },
                text_parser::ParsedObject::Parenthesis { 
                    inner, 
                    parenthesis_type, 
                    super_script, 
                    sub_script 
                } => {
                },
            }
        }


        return Ok(KElement::LinearGroup(root));
    }
    
}