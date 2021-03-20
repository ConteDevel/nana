pub struct Fsm<TFunc> {
    memory: Vec<TFunc>
}

impl<TFunc> Fsm<TFunc> {

    pub fn new(state: TFunc) -> Self {
        Self {
            memory: vec![state]
        }
    }

    pub fn push(&mut self, state: TFunc) {
        self.memory.push(state)
    }

    pub fn pop(&mut self) -> Option<TFunc> {
        return self.memory.pop();
    }

    pub fn top(&mut self) -> Option<&TFunc> {
        return self.memory.last();
    }
}