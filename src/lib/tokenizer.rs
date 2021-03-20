use crate::fsm::Fsm;

type State = fn(&mut Tokenizer, &str, &mut usize) -> Option<String>;

pub struct Tokenizer {
    fsm: Fsm<State>
}

impl Tokenizer {

    pub fn new() -> Self {
        Self {
            fsm: Fsm::<State>::new(Self::init)
        }
    }

    pub fn next(&mut self, source: &str, mut idx: &mut usize) -> Option<String> {
        if let Some(state) = self.fsm.top() {
            return state(self, &source, &mut idx);
        }
        return None;
    }

    fn init(tokenizer: &mut Tokenizer, source: &str, mut idx: &mut usize) -> Option<String> {
        match source.chars().nth(*idx) {
            Some('a'..='z'|'A'..='Z') => tokenizer.fsm.push(Tokenizer::name),
            Some('\'') => tokenizer.fsm.push(Tokenizer::string),
            Some(_) => tokenizer.fsm.push(Tokenizer::special),
            _ => return None
        }
        return tokenizer.next(&source, &mut idx);
    }

    fn string(tokenizer: &mut Tokenizer, source: &str, idx: &mut usize) -> Option<String> {
        tokenizer.fsm.pop();
        let start = *idx;
        *idx += 1;
        while let Some(ch) = source.chars().nth(*idx) {
            *idx += 1;
            if ch == '\'' {
                return Some(source.chars().skip(start).take(*idx - start).collect())
            }
        }
        return None;
    }

    fn special(tokenizer: &mut Tokenizer, source: &str, idx: &mut usize) -> Option<String> {
        tokenizer.fsm.pop();
        let result = Some(String::from(source.chars().nth(*idx).unwrap()));
        *idx += 1;
        return result;
    }

    fn name(tokenizer: &mut Tokenizer, source: &str, idx: &mut usize) -> Option<String> {
        tokenizer.fsm.pop();
        let start = *idx;
        *idx += 1;
        while let Some('a'..='z'|'A'..='Z') = source.chars().nth(*idx) {
            *idx += 1
        }
        return Some(source.chars().skip(start).take(*idx - start).collect())
    }
}