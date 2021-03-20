use crate::fsm::Fsm;

type State = fn(&str, &mut usize) -> Option<String>;

pub struct Tokenizer {
    fsm: Fsm<State>
}

fn init(source: &str, idx: &mut usize) -> Option<String> {
    return None;
}

impl Tokenizer {

    pub fn new() -> Self {
        Self {
            fsm: Fsm::<State>::new(init)
        }
    }

    pub fn next(&mut self, source: &str, mut idx: &mut usize) -> Option<String> {
        if let Some(state) = self.fsm.top() {
            return state(&source, &mut idx);
        }
        return None;
    }
}