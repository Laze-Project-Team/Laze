use std::{
    io::{stderr, Write},
    rc::Rc,
};

use crate::{Parser, ParserData};

pub type Matcher<T> = Rc<dyn Fn(&mut Parser<T>) -> Result<String, ()>>;

pub fn parse_str<T: ParserData + Clone + 'static>(str: String) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_str {:?}", str);
        if parser.input.starts_with(&*str) {
            parser.eat(&str);
            Ok(str.to_string())
        } else {
            Err(())
        }
    });
}

pub fn parse_any<T: ParserData + Clone + 'static>() -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        println!("parse_any {}", parser.input);
        if parser.input.len() > 0 {
            let ch = parser.input.chars().nth(0).unwrap();
            if ch == '\n' {
                return Err(());
            }
            parser.eat(&ch.to_string());
            Ok(ch.to_string())
        } else {
            Err(())
        }
    });
}

fn get_char_range(range: String) -> Vec<char> {
    assert!(range.contains('-'));
    let mut range_chars = Vec::<char>::new();
    let mut str_iter = range.chars().into_iter().peekable();
    while let Some(c) = str_iter.next() {
        match c {
            '\\' => {}
            cbegin => match str_iter.peek() {
                Some('-') => {
                    str_iter.next();
                    match str_iter.peek() {
                        Some(&cend) => {
                            str_iter.next();
                            range_chars
                                .append(&mut (cbegin..cend).into_iter().collect::<Vec<char>>());
                            range_chars.push(cend);
                        }
                        None => {
                            let _ = writeln!(stderr(), "Invalid range starting with {}.", cbegin);
                            return vec![];
                        }
                    }
                }
                _ => range_chars.push(cbegin),
            },
        }
    }
    range_chars
}

pub fn parse_range<T: ParserData + Clone + 'static>(range: String) -> Matcher<T> {
    let range_chars: Vec<char>;
    if range.contains('-') {
        range_chars = get_char_range(range);
    } else {
        range_chars = range.chars().collect();
    }
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_range");
        if range_chars.contains(match &parser.input.chars().nth(0) {
            Some(c) => c,
            None => return Err(()),
        }) {
            let ch = parser.input.chars().nth(0).unwrap();
            parser.eat(&ch.to_string());
            Ok(ch.to_string())
        } else {
            Err(())
        }
    });
}

pub fn parse_many<T: ParserData + Clone + 'static>(matcher: Matcher<T>) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_many");
        let mut matched = "".to_string();
        while let Ok(str) = matcher(parser) {
            matched += str.as_str();
        }
        Ok(matched)
    });
}

pub fn parse_more_than_one<T: ParserData + Clone + 'static>(matcher: Matcher<T>) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_more_than_one");
        let mut matched = "".to_string();
        if let Ok(str) = matcher(parser) {
            matched += str.as_str();
            if let Ok(str) = parse_many(matcher.clone())(parser) {
                matched += str.as_str();
                Ok(matched)
            } else {
                Ok(matched)
            }
        } else {
            Err(())
        }
    });
}

pub fn parse_not<T: ParserData + Clone + 'static>(matcher: Matcher<T>) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_not");
        if let Ok(str) = matcher(parser) {
            parser.input = str + &parser.input;
            Err(())
        } else {
            Ok("".to_string())
        }
    });
}

pub fn parse_seq<T: ParserData + Clone + 'static>(matchers: Vec<Matcher<T>>) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_seq");
        let mut matched = "".to_string();
        for matcher in &matchers {
            match matcher(parser) {
                Ok(str) => {
                    // println!("parse_seq: {:?}", str);
                    matched += str.as_str();
                }
                Err(()) => {
                    // add the eaten letters to the input string
                    parser.input = matched + &parser.input;
                    return Err(());
                }
            }
        }
        Ok(matched)
    });
}

pub fn parse_or<T: ParserData + Clone + 'static>(matchers: Vec<Matcher<T>>) -> Matcher<T> {
    // backtrack needed
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        // println!("parse_or");
        for matcher in &matchers {
            let mut temp = parser.clone();
            let matched: String;
            // println!("parse_or: {}", parser.input);
            match matcher(&mut temp) {
                Ok(str) => {
                    matched = str;
                }
                Err(()) => continue,
            }
            *parser = temp.clone();
            return Ok(matched);
        }
        Err(())
    });
}

pub fn parse_ref<T: ParserData + Clone + 'static>(
    name: String,
    save_name: Option<String>,
) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        println!("parse_ref {}", name);
        let matcher: Matcher<T>;
        if let Some(m) = parser.grammar_list.get(name.as_str()) {
            matcher = m.clone();
        } else {
            panic!("Could not find {} in the grammar.", name);
        }
        parser.enter_scope();
        match matcher(parser) {
            Ok(str) => {
                // println!("parsed: {str}");
                let data = T::data(name.clone(), parser);
                parser.exit_scope();
                match save_name.clone() {
                    Some(str) => parser.add_data(str.clone(), data),
                    None => parser.add_data(name.clone(), data),
                }
                Ok(str)
            }
            Err(()) => {
                parser.exit_scope();
                Err(())
            }
        }
    });
}

pub fn capture_string<T: ParserData + Clone + 'static>(
    name: String,
    matcher: Matcher<T>,
) -> Matcher<T> {
    return Rc::new(move |parser: &mut Parser<T>| -> Result<String, ()> {
        match matcher(parser) {
            Ok(str) => {
                parser.add_data(name.clone(), T::string(str.clone()));
                Ok(str)
            }
            Err(()) => Err(()),
        }
    });
}
