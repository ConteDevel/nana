#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Position {
    start: usize,
    end: usize,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum EToken {
    Unrecognized(Position),
    Assignment(Position),
    Colon(Position),
    Id(String, Position),
    Uint(u64, Position),
    NewLine(Position),
    Str(String, Position),
}

#[derive(Debug)]
pub struct Tokenizer {

}

impl Position {
    pub fn new(start: usize, end: usize) -> Self {
        Position {
            start: start,
            end: end
        }
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&self, source: &str, mut idx: &mut usize) -> Option<EToken> {
        match source.chars().nth(*idx) {
            Some('0'..='9') => self.uint(&source, &mut idx),
            Some('=') => Some(EToken::Assignment(Position::new(*idx, *idx + 1))),
            Some(':') => Some(EToken::Colon(Position::new(*idx, *idx + 1))),
            Some('\n') => self.new_line(&source, &mut idx),
            Some('_'|'a'..='z'|'A'..='Z') => self.id(&source, &mut idx),
            Some('\''|'"') => self.str(&source, &mut idx),
            Some(_) => Some(EToken::Unrecognized(Position::new(*idx, *idx + 1))),
            _ => return None
        }
    }

    fn id(&self, source: &str, idx: &mut usize) -> Option<EToken> {
        let start = *idx;
        *idx += 1;
        while let Some('_'|'a'..='z'|'A'..='Z'|'0'..='9') = source.chars().nth(*idx) {
            *idx += 1
        }
        let id: String = source.chars().skip(start).take(*idx - start).collect();
        return Some(EToken::Id(id, Position::new(start, *idx)))
    }

    fn new_line(&self, source: &str, idx: &mut usize) -> Option<EToken> {
        let start = *idx;
        *idx += 1;
        while let Some('\n') = source.chars().nth(*idx) {
            *idx += 1
        }
        return Some(EToken::NewLine(Position::new(start, *idx)))
    }

    fn str(&self, source: &str, idx: &mut usize) -> Option<EToken> {
        let term = source.chars().nth(*idx);
        let start = *idx;
        *idx += 1;
        while term != source.chars().nth(*idx) || '\\' == source.chars().nth(*idx - 1).unwrap_or_default() {
            if source.chars().nth(*idx).is_none() {
                return Some(EToken::Unrecognized(Position::new(start, *idx)))
            }
            *idx += 1
        }
        let value: String = source.chars().skip(start + 1).take(*idx - start - 1).collect();
        return Some(EToken::Str(value, Position::new(start, *idx + 1)))
    }

    fn uint(&self, source: &str, idx: &mut usize) -> Option<EToken> {
        let start = *idx;
        *idx += 1;
        while let Some('0'..='9') = source.chars().nth(*idx) {
            *idx += 1
        }
        return match source.chars().skip(start).take(*idx - start).collect::<String>().parse::<u64>() {
            Ok(value) => Some(EToken::Uint(value, Position::new(start, *idx))),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id() {
        let tokenizer = Tokenizer::new();
        let mut idx = 0;
        let mut source = "0a";
        let mut token = tokenizer.id(&source, &mut idx);
        assert_eq!(token, Some(EToken::Id(String::from("0a"), Position::new(0, 2))));
        idx = 0;
        source = "_testVar1 ";
        token = tokenizer.id(&source, &mut idx);
        assert_eq!(token, Some(EToken::Id(String::from("_testVar1"), Position::new(0, 9))));
        idx = 0;
        source = "_ testVar";
        token = tokenizer.id(&source, &mut idx);
        assert_eq!(token, Some(EToken::Id(String::from("_"), Position::new(0, 1))));
    }

    #[test]
    fn next() {
        let tokenizer = Tokenizer::new();
        let mut idx = 0;
        let mut source = "";
        let mut token = tokenizer.next(&source, &mut idx);
        assert_eq!(token, None);
        idx = 0;
        source = "";
        token = tokenizer.next(&source, &mut idx);
        assert_eq!(token, None);
    }

    #[test]
    fn new_line() {
        let tokenizer = Tokenizer::new();
        let mut idx = 0;
        let mut source = "\n";
        let mut token = tokenizer.new_line(&source, &mut idx);
        assert_eq!(token, Some(EToken::NewLine(Position::new(0, 1))));
        idx = 0;
        source = "\n\n\n ";
        token = tokenizer.new_line(&source, &mut idx);
        assert_eq!(token, Some(EToken::NewLine(Position::new(0, 3))));
    }

    #[test]
    fn str() {
        let tokenizer = Tokenizer::new();
        let mut idx = 0;
        let mut source = "''";
        let mut token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::new(), Position::new(0, 2))));
        idx = 0;
        source = "\"\"";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::new(), Position::new(0, 2))));
        idx = 0;
        source = "\"'\"";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::from("'"), Position::new(0, 3))));
        idx = 0;
        source = "'\"'";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::from("\""), Position::new(0, 3))));
        idx = 0;
        source = "'\\''";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::from("\\'"), Position::new(0, 4))));
        idx = 0;
        source = "\"\\\"\"";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::from("\\\""), Position::new(0, 4))));
        idx = 0;
        source = "\"Jack's home\"";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Str(String::from("Jack's home"), Position::new(0, 13))));
        idx = 0;
        source = "\"Jack's home";
        token = tokenizer.str(&source, &mut idx);
        assert_eq!(token, Some(EToken::Unrecognized(Position::new(0, 12))));
    }

    #[test]
    fn uint() {
        let tokenizer = Tokenizer::new();
        let mut idx = 0;
        let mut source = "5";
        let mut token = tokenizer.uint(&source, &mut idx);
        assert_eq!(token, Some(EToken::Uint(5, Position::new(0, 1))));
        idx = 0;
        source = "0123456789 1";
        token = tokenizer.uint(&source, &mut idx);
        assert_eq!(token, Some(EToken::Uint(123456789, Position::new(0, 10))));
    }
}