// #![allow(warnings)]
use std::{fs, str::Chars};


fn main() {
    let source = fs::read_to_string("./index.js").unwrap().replace("\r", "");
    
    let result = init(source.chars(),State { current: vec![], source: String::new()});
    
    fs::write("./dist.js", result.source).unwrap();
}

struct State {
    current: Vec<Element>,
    source: String
}

#[allow(dead_code)]
#[derive(Debug)]
struct Element {
    tag: String,
    inner: String,
    attr: Vec<(String,String)>,
    childs: Vec<Element>
}

impl Element {
    fn new(tag: String) -> Self { Self { tag, attr: vec![], childs: vec![], inner: String::new() } }
    fn text(content: String) -> Self {
        Self {
            attr: vec![],
            childs: vec![],
            inner: content,
            tag: "TEXT".to_string()
        }
    }
}

fn resolve(elem: &Element) -> String {
    let mut out = String::from("createElement(");
    let tag = elem.tag.clone();
    let tag = if tag.chars().next().unwrap().is_uppercase() { tag } else { format!("\"{}\"",tag) };
    
    out.push_str( 
        format!("{}, {}, {})",
        tag,
        
        if elem.attr.is_empty() { "null".to_string() } else {
            format!("{{ {} }}",elem.attr.iter().map(|e|{
                if e.0 == "_JSX" {
                    return format!("{},",e.1)
                }
                return format!("{}:{},",e.0,e.1)
            }).collect::<Vec<String>>().join(""))
        },
        
        if elem.childs.is_empty() { "".to_string() } else {
            format!("{}",elem.childs.iter().map(|e| -> String{
                if e.tag == "TEXT".to_string() {
                    return format!("{}", e.inner.clone().replace("\t", "\\t").replace("\n", "\\n") )
                }
                resolve(e)
            }).collect::<Vec<String>>().join(", "))
        }
    ).as_str() );
    
    out
}

fn init(mut iter: Chars,mut state: State) -> State {
    loop {
        match iter.next() {
            Some('<') => return identity(iter, state),
            Some(ch) => state.source.push(ch),
            None => return state
        }
    }
}

fn identity(mut iter: Chars, mut state: State) -> State {
    let mut id = String::new();
    let mut is_close = 0;
    
    loop {
        match iter.next() {
            Some('/') => {                
                is_close = if id.is_empty() { 1 } else { 2 };
                id.clear();
            }    
            
            // collector
            Some(ch) if ch.is_alphabetic() => id.push(ch),
            
            // mutating
            Some(ch) if ch.is_whitespace() => {
                state.current.push(Element::new(id));
                return attribute(iter, state)
            }
            
            // mutating
            Some('>') if is_close != 0 => {
                
                let child = if is_close == 1 { state.current.pop().unwrap() } else { Element::new(id) };
                
                match state.current.last_mut() {
                    Some(pr) => pr.childs.push(child),
                    None => state.source.push_str( resolve(&child).as_str() )
                }
                return init(iter, state);
            }
            
            // mutating
            Some('>') => {
                state.current.push(Element::new(id));
                return inner(iter, state)
            }
                        
            Some(_) => todo!(),
            None => return state,
        }
    }
}

fn attribute(mut iter: Chars, mut state: State) -> State {
    let mut key = String::new();
    let mut val = String::new();
    let mut is_key: u8 = 0;
    loop {
        match iter.next() {
            // key collector
            Some(ch) if ch.is_whitespace() => if is_key == 1 { is_key = 2; },
            
            // mutating
            Some('{') if is_key == 0 || is_key == 3 =>{
                let out = collect_exp(&mut iter);
                state.current.last_mut().unwrap().attr.push((
                    if key.is_empty() { "_JSX".to_string() } else { key.clone() },
                    out
                ));
                key.clear();
                val.clear();
                is_key = 0;
            },
            Some(ch) if is_key == 0 && ch.is_alphabetic() => {
                key.push(ch);
                is_key = 1;
            },
            
            Some('/') if is_key == 0 => {
                let child = state.current.pop().unwrap();
                
                match state.current.last_mut() {
                    Some(pr) => pr.childs.push(child),
                    None => state.source.push_str( resolve(&child).as_str() )
                }
                loop {
                    match iter.next() {
                        Some('>') => break,
                        Some(_) => todo!(),
                        None => todo!(),
                    }
                }
                return init(iter, state);
            },
            
            Some(ch) if is_key == 9 => val.push(ch),
            
            
            Some('=') if is_key == 1 || is_key == 2 => is_key = 3,
            Some(ch) if is_key == 1 => key.push(ch),
            
            Some('"') if is_key == 3 => {
                val.push('"');
                is_key = 4;
            },
            // mutating
            Some('"') if is_key == 4 => {
                val.push('"');
                state.current.last_mut().unwrap().attr.push((key.clone(),val.clone()));
                key.clear();
                val.clear();
                is_key = 0;
            }
            
            Some(ch) if is_key == 4 => val.push(ch),
            
            Some('>') => {
                return inner(iter, state);
            }
            
            Some(ch) => panic!("{ch}{}{}{}{}{}",
                iter.next().unwrap_or_default(),
                iter.next().unwrap_or_default(),
                iter.next().unwrap_or_default(),
                iter.next().unwrap_or_default(),
                iter.next().unwrap_or_default()
            ),
            None => return state,
        };
    }
}

fn inner(mut iter: Chars, mut state: State) -> State {
    let mut inner = String::new();
    
    loop {
        match iter.next() {
            
            // mutating
            Some('<') => {
                if !inner.is_empty() {
                    state.current.last_mut().unwrap().childs.push( Element::text(format!("\"{}\"",inner)) );
                }
                return identity(iter, state);
            }
            
            // collector
            Some('{') => {
                if !inner.is_empty() {
                    state.current.last_mut().unwrap().childs.push( Element::text(format!("\"{}\"",inner)) );
                }
                inner.clear();
                let exp = collect_exp(&mut iter);
                state.current.last_mut().unwrap().childs.push( Element::text(exp) );
            },
            Some(ch) => inner.push(ch),
            
            None => return state,
        };
    }
}

fn collect_exp(iter: &mut Chars) -> String {
    let mut out = String::new();
    let mut scope: u8 = 0;
    
    loop {
        match iter.next() {
            Some('{') => {
                out.push('{');
                scope += 1;
            },
            Some('}') if scope == 0 => {
                return out
            },
            Some('}') => {
                out.push('}');
                scope -= 1;
            }
            
            Some(ch) => out.push(ch),
            None => panic!("Unexpected end of file"),
        }
    }
}