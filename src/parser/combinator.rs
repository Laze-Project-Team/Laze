use std::{
    collections::HashMap,
    io::{stderr, Write},
    rc::Rc,
};

use super::parser::Parser;

pub type Matcher<'a, T> = Rc<dyn Fn(Parser<'a, T>) -> Option<Parser<'a, T>> + 'a>;

pub fn parse_str<'a, T: Clone>(str: &'a str) -> Matcher<'a, T> {
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        if parser.input.starts_with(&*str) {
            Some(Parser {
                input: parser
                    .input
                    .chars()
                    .skip(str.chars().count())
                    .into_iter()
                    .collect::<String>(),
                ..parser
            })
        } else {
            None
        }
    });
}

pub fn parse_any<'a, T: Clone>() -> Matcher<'a, T> {
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        if parser.input.len() > 0 {
            Some(Parser {
                input: parser.input.chars().skip(1).into_iter().collect::<String>(),
                ..parser
            })
        } else {
            None
        }
    });
}

fn get_char_range(range: &str) -> Vec<char> {
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

pub fn parse_range<'a, T: Clone>(range: &'a str) -> Matcher<'a, T> {
    let range_chars: Vec<char>;
    if range.contains('-') {
        range_chars = get_char_range(range);
    } else {
        range_chars = range.chars().collect();
    }
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        if range_chars.contains(match &parser.input.chars().nth(0) {
            Some(c) => c,
            None => return None,
        }) {
            Some(Parser {
                input: parser.input.chars().skip(1).into_iter().collect::<String>(),
                ..parser
            })
        } else {
            None
        }
    });
}

pub fn parse_many<'a, T: Clone + 'a>(matcher: Matcher<'a, T>) -> Matcher<'a, T> {
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        let mut p = parser;
        if let Some(newp) = matcher(p.clone()) {
            p = newp;
            while let Some(newp) = matcher(p.clone()) {
                p = newp;
            }
            return Some(p);
        } else {
            return Some(p);
        }
    });
}

pub fn parse_more_than_one<'a, T: Clone + 'a>(matcher: Matcher<'a, T>) -> Matcher<'a, T> {
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        if let Some(newp) = matcher(parser) {
            if let Some(newp) = parse_many(matcher.clone())(newp) {
                Some(newp)
            } else {
                None
            }
        } else {
            None
        }
    });
}

pub fn parse_not<'a, T: Clone + 'a>(matcher: Matcher<'a, T>) -> Matcher<'a, T> {
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        if let Some(_) = matcher(parser.clone()) {
            return None;
        } else {
            return Some(parser);
        }
    });
}

pub fn parse_seq<'a, T: Clone + 'a>(matchers: Vec<Matcher<'a, T>>) -> Matcher<'a, T> {
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        let mut p = parser;
        for matcher in &matchers {
            match matcher(p) {
                Some(newp) => {
                    p = newp;
                }
                None => return None,
            }
        }
        return Some(p);
    });
}

pub fn parse_or<'a, T: Clone + 'a>(matchers: Vec<Matcher<'a, T>>) -> Matcher<'a, T> {
    // backtrack needed
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        for matcher in &matchers {
            match matcher(parser.clone()) {
                Some(p) => {
                    return Some(p);
                }
                None => continue,
            }
        }
        return None;
    });
}

pub fn parse_ref<'a, T: Clone + 'a>(
    peg: HashMap<&'a str, Matcher<'a, T>>,
    name: &str,
) -> Matcher<'a, T> {
    let matcher: Option<Matcher<'a, T>>;
    if peg.contains_key(name) {
        matcher = Some(peg[name].clone());
    } else {
        let _ = writeln!(stderr(), "Could not find {} in the grammar.", name);
        matcher = None;
    };
    return Rc::new(move |parser: Parser<'a, T>| -> Option<Parser<'a, T>> {
        if let Some(m) = &matcher {
            match m(parser) {
                Some(p) => {
                    return Some(p);
                }
                None => {
                    return None;
                }
            }
        } else {
            None
        }
    });
}

// pub fn CaptureGroup<'a>(name: &str, matcher: Matcher<'a>) -> Matcher<'a> {}
