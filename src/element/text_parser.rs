// use regex::{Captures, Regex};
// use lazy_static::lazy_static;
// use std::collections::VecDeque;

// lazy_static! {
//     static ref FUNCTION_REGEX: Regex = Regex::new(r"\\([a-zA-Z]+)(\s*(?:\{[^}]*\}|[^_^{}\s\\])*)\s*(?:_(\{[^}]*\}|[^_^{}\s\\]))?\s*(?:\^(\{[^}]*\}|[^_^{}\s\\]))?").unwrap();
//     static ref TEXT_REGEX: Regex = Regex::new(r"^\{\}<>()\[\]]+").unwrap();
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum PerenthesisType {
//     Round,    // ()
//     Square,   // []
//     // Curly,    // {}
//     // Angle,    // <>
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum ParsedObject {
//     Func {
//         name: String,
//         content: Vec<Vec<ParsedObject>>,
//         super_script: Vec<ParsedObject>,
//         sub_script: Vec<ParsedObject>,
//     },
//     Var {
//         text: String,
//         super_script: Vec<ParsedObject>,
//         sub_script: Vec<ParsedObject>,
//     },
//     Operator {
//         text: String,
//     },
//     Parenthesis {
//         inner: Vec<ParsedObject>,
//         parenthesis_type: PerenthesisType,
//         super_script: Vec<ParsedObject>,
//         sub_script: Vec<ParsedObject>,
//     },
// }

// pub fn parse(input: &str) -> Result<Vec<ParsedObject>, String> {
//     if input.is_empty() {
//         return Ok(Vec::new())
//     }

//     let mut i = 0;

//     let mut objects = Vec::new();

//     for i in 0..input.len() {
//         // let mut functions = FUNCTION_REGEX.is_match_at();
//         // FUNCTION_REGEX.(&mut functions, input);
//         // for i in 0..functions.len() {
//         //     println!("{:?}", functions.get(i));
//         // }

//         if TEXT_REGEX.is_match_at(input, i) {
//             println!("TEXT!")
//         } else if FUNCTION_REGEX.is_match_at(input, i) {
//             println!("FUNCTION!")
//         } else {
//             break;
//         }
//     }



//     // loop {
        
//     // }

//     // for captures in FUNCTION_REGEX.captures_iter(input) {
//     //     objects.push(parse_function(&captures)?);
//     // }

//     // println!("{:?}", objects);

//     // let capture_locations = FUNCTION_REGEX.captures_read(locs, haystack);

//     // loop {
        
//     // }


//     Ok(objects)
// }

// fn parse_function(captures: &Captures) -> Result<(usize, ParsedObject), String> {
//     let name = captures.get(1).unwrap().as_str();
//     let args_str = captures.get(2).map_or("", |m| m.as_str());
//     let sub = captures.get(3).map(|m| m.as_str());
//     let super_script = captures.get(4).map(|m| m.as_str());

//     let len = captures.iter().map(|c| c.iter().len()).sum::<usize>();
    
//     // Parse arguments
//     let mut args = Vec::new();
//     let args_clean = args_str.trim();
//     if !args_clean.is_empty() {
//         let mut i = 0;
//         let chars: Vec<char> = args_clean.chars().collect();
//         while i < chars.len() {
//             if chars[i] == '{' {
//                 let mut brace_content = String::new();
//                 i += 1;
//                 while i < chars.len() && chars[i] != '}' {
//                     brace_content.push(chars[i]);
//                     i += 1;
//                 }
//                 args.push(brace_content);
//                 i += 1;
//             } else if chars[i] != ' ' {
//                 args.push(chars[i].to_string());
//                 i += 1;
//             } else {
//                 i += 1;
//             }
//         }
//     }
    
//     // Clean up sub/super (remove braces if present)
//     let sub_clean = sub.map(|s| if s.starts_with('{') && s.ends_with('}') {
//         &s[1..s.len()-1]
//     } else { s });
//     let super_clean = super_script.map(|s| if s.starts_with('{') && s.ends_with('}') {
//         &s[1..s.len()-1] 
//     } else { s });
    
//     Ok(
//         (
//         len,
//         ParsedObject::Func { 
//             name: name.to_string(), 
//             content: parse_string_arr(&args)?, 
//             super_script: if let Some(super_clean) = super_clean {
//                 parse(super_clean)?
//             } else {
//                 Vec::new()
//             }, 
//             sub_script: if let Some(sub_clean) = sub_clean {
//                 parse(sub_clean)?
//             } else {
//                 Vec::new()
//             }, 
//         }
//     ))
// }

// fn parse_string_arr(args: &Vec<String>) -> Result<Vec<Vec<ParsedObject>>, String> {
//     let mut result_vec = Vec::new();
//     for arg in args {
//         result_vec.push(parse(arg)?);
//     }
//     Ok(result_vec)
// }



use regex::Regex;
use lazy_static::lazy_static;
use std::collections::VecDeque;

lazy_static! {
    static ref FUNCTION_REGEX: Regex = Regex::new(r"\\[a-zA-Z]+").unwrap();
    static ref NUMBER_REGEX: Regex = Regex::new(r"\d+(?:\.\d+)?").unwrap();
    static ref LETTER_REGEX: Regex =  Regex::new(r"[a-zA-Z]").unwrap();
    static ref OPERATOR_REGEX: Regex = Regex::new(r"[+\-=*/±]").unwrap();
    static ref WHITESPACE_REGEX: Regex = Regex::new(r"\s+").unwrap();
}

#[derive(Debug, Clone, PartialEq)]
pub enum PerenthesisType {
    Round,    // ()
    Square,   // []
    // Curly,    // {}
    // Angle,    // <>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedObject {
    Func {
        name: String,
        content: Vec<Vec<ParsedObject>>,
        super_script: Vec<ParsedObject>,
        sub_script: Vec<ParsedObject>,
    },
    Var {
        text: String,
        super_script: Vec<ParsedObject>,
        sub_script: Vec<ParsedObject>,
    },
    Operator {
        text: String,
    },
    Parenthesis {
        inner: Vec<ParsedObject>,
        parenthesis_type: PerenthesisType,
        super_script: Vec<ParsedObject>,
        sub_script: Vec<ParsedObject>,
    },
}

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Function(String),
    Number(String),
    Letter(String),
    Operator(String),
    LeftParen(PerenthesisType),
    RightParen(PerenthesisType),
    LeftBrace,
    RightBrace,
    Superscript,
    Subscript,
}

pub fn parse(input: &str) -> Result<Vec<ParsedObject>, String> {
    let tokens = tokenize_with_regex(input)?;
    let mut token_queue = VecDeque::from(tokens);
    parse_tokens(&mut token_queue)
}

fn tokenize_with_regex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let input_chars: Vec<char> = input.chars().collect();

    while pos < input_chars.len() {
        let remaining: String = input_chars[pos..].iter().collect();
        
        // Skip whitespace
        if let Some(mat) = WHITESPACE_REGEX.find(&remaining) {
            if mat.start() == 0 {
                pos += mat.len();
                continue;
            }
        }

        // Check for TeX functions
        if let Some(mat) = FUNCTION_REGEX.find(&remaining) {
            if mat.start() == 0 {
                let func_name = mat.as_str()[1..].to_string(); // Remove the \
                tokens.push(Token::Function(func_name));
                pos += mat.len();
                continue;
            }
        }

        // Check for numbers (including decimals)
        if let Some(mat) = NUMBER_REGEX.find(&remaining) {
            if mat.start() == 0 {
                tokens.push(Token::Number(mat.as_str().to_string()));
                pos += mat.len();
                continue;
            }
        }

        // Check for single letters
        if let Some(mat) = LETTER_REGEX.find(&remaining) {
            if mat.start() == 0 {
                tokens.push(Token::Letter(mat.as_str().to_string()));
                pos += mat.len();
                continue;
            }
        }

        // Check for operators
        if let Some(mat) = OPERATOR_REGEX.find(&remaining) {
            if mat.start() == 0 {
                tokens.push(Token::Operator(mat.as_str().to_string()));
                pos += mat.len();
                continue;
            }
        }

        // Handle special characters
        match input_chars[pos] {
            '(' => tokens.push(Token::LeftParen(PerenthesisType::Round)),
            ')' => tokens.push(Token::RightParen(PerenthesisType::Round)),
            '[' => tokens.push(Token::LeftParen(PerenthesisType::Square)),
            ']' => tokens.push(Token::RightParen(PerenthesisType::Square)),
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            '^' => tokens.push(Token::Superscript),
            '_' => tokens.push(Token::Subscript),
            _ => return Err(format!("Unexpected character: '{}'", input_chars[pos])),
        }
        
        pos += 1;
    }

    Ok(tokens)
}

fn parse_tokens(tokens: &mut VecDeque<Token>) -> Result<Vec<ParsedObject>, String> {
    let mut result = Vec::new();

    while let Some(token) = tokens.pop_front() {
        match token {
            Token::Function(name) => {
                let func = parse_function_generic(name, tokens)?;
                result.push(func);
            }
            Token::Number(text) | Token::Letter(text) => {
                let var = parse_variable(text, tokens)?;
                result.push(var);
            }
            Token::Operator(op) => {
                result.push(ParsedObject::Operator { text: op });
            }
            Token::LeftParen(paren_type) => {
                let paren = parse_parenthesis(paren_type, tokens)?;
                result.push(paren);
            }
            Token::RightParen(_) | Token::RightBrace => {
                // Put it back - should be handled by parent context
                tokens.push_front(token);
                break;
            }
            _ => {
                tokens.push_front(token);
                break;
            }
        }
    }

    Ok(result)
}

fn parse_function_generic(name: String, tokens: &mut VecDeque<Token>) -> Result<ParsedObject, String> {
    let mut content = Vec::new();
    
    // Parse all braced content that follows this function
    while let Some(Token::LeftBrace) = tokens.front() {
        let braced_content = parse_braced_content(tokens)?;
        content.push(braced_content);
    }

    // Parse potential superscript and subscript
    let (super_script, sub_script) = parse_scripts(tokens)?;

    Ok(ParsedObject::Func {
        name,
        content,
        super_script,
        sub_script,
    })
}

fn parse_variable(text: String, tokens: &mut VecDeque<Token>) -> Result<ParsedObject, String> {
    let (super_script, sub_script) = parse_scripts(tokens)?;

    Ok(ParsedObject::Var {
        text,
        super_script,
        sub_script,
    })
}

fn parse_parenthesis(paren_type: PerenthesisType, tokens: &mut VecDeque<Token>) -> Result<ParsedObject, String> {
    let mut inner = Vec::new();

    // Parse until we find the matching closing parenthesis
    while !tokens.is_empty() {
        if let Some(Token::RightParen(closing_type)) = tokens.front() {
            if *closing_type == paren_type {
                tokens.pop_front(); // consume the closing paren
                break;
            }
        }
        
        let parsed = parse_tokens(tokens)?;
        inner.extend(parsed);
        
        if tokens.is_empty() {
            return Err("Unmatched opening parenthesis".to_string());
        }
    }

    let (super_script, sub_script) = parse_scripts(tokens)?;

    Ok(ParsedObject::Parenthesis {
        inner,
        parenthesis_type: paren_type,
        super_script,
        sub_script,
    })
}

fn parse_braced_content(tokens: &mut VecDeque<Token>) -> Result<Vec<ParsedObject>, String> {
    if let Some(Token::LeftBrace) = tokens.pop_front() {
        let mut content = Vec::new();

        while !tokens.is_empty() {
            if let Some(Token::RightBrace) = tokens.front() {
                tokens.pop_front(); // consume the closing brace
                break;
            }
            
            let parsed = parse_tokens(tokens)?;
            content.extend(parsed);
            
            if tokens.is_empty() {
                return Err("Unmatched opening brace".to_string());
            }
        }

        Ok(content)
    } else {
        Err("Expected opening brace".to_string())
    }
}

fn parse_scripts(tokens: &mut VecDeque<Token>) -> Result<(Vec<ParsedObject>, Vec<ParsedObject>), String> {
    let mut super_script = Vec::new();
    let mut sub_script = Vec::new();

    // Parse superscript and subscript (can appear in any order)
    while let Some(token) = tokens.front() {
        match token {
            Token::Superscript => {
                tokens.pop_front(); // consume ^
                super_script = parse_script_content(tokens)?;
            }
            Token::Subscript => {
                tokens.pop_front(); // consume _
                sub_script = parse_script_content(tokens)?;
            }
            _ => break,
        }
    }

    Ok((super_script, sub_script))
}

fn parse_script_content(tokens: &mut VecDeque<Token>) -> Result<Vec<ParsedObject>, String> {
    if let Some(token) = tokens.front() {
        match token {
            Token::LeftBrace => {
                // Multi-character script content in braces
                parse_braced_content(tokens)
            }
            Token::Number(text) | Token::Letter(text) => {
                // Single character/number script content
                let text = text.clone();
                tokens.pop_front();
                Ok(vec![ParsedObject::Var {
                    text,
                    super_script: vec![],
                    sub_script: vec![],
                }])
            }
            Token::Function(name) => {
                // Function in script
                let name = name.clone();
                tokens.pop_front();
                let func = parse_function_generic(name, tokens)?;
                Ok(vec![func])
            }
            _ => Err("Invalid script content".to_string()),
        }
    } else {
        Err("Expected script content".to_string())
    }
}