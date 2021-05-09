use std::fmt;

#[derive(Debug)]
pub enum EToken {
    Id(usize),
    Int(i64),
    Pls,
    Mns,
    Div,
    Mul,
}

pub struct Tokenizer {
}

impl fmt::Display for EToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Tokenizer {

    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&mut self, source: &str, mut idx: &mut usize) -> Option<EToken> {
        match source.chars().nth(*idx) {
            Some('0'..='9') => self.uint(&source, &mut idx),
            Some('+'|'-'|'*'|'/') => self.operator(&source, &mut idx),
            Some(_) => self.name(&source, &mut idx),
            _ => return None
        }
    }

    fn operator(&mut self, source: &str, idx: &mut usize) -> Option<EToken> {
        let result = match source.chars().nth(*idx) {
            Some('+') => Some(EToken::Pls),
            Some('-') => Some(EToken::Mns),
            Some('*') => Some(EToken::Mul),
            Some('/') => Some(EToken::Div),
            _ => None
        };
        *idx += 1;
        return result
    }

    fn uint(&mut self, source: &str, idx: &mut usize) -> Option<EToken> {
        let start = *idx;
        *idx += 1;
        while let Some('0'..='9') = source.chars().nth(*idx) {
            *idx += 1
        }
        return match source.chars().skip(start).take(*idx - start).collect::<String>().parse::<i64>() {
            Ok(value) => Some(EToken::Int(value)),
            _ => None
        }
    }

    fn name(&mut self, source: &str, idx: &mut usize) -> Option<EToken> {
        // let start = *idx;
        *idx += 1;
        while let Some('a'..='z'|'A'..='Z') = source.chars().nth(*idx) {
            *idx += 1
        }
        // return Some(source.chars().skip(start).take(*idx - start).collect())
        return Some(EToken::Id(1))
    }
}