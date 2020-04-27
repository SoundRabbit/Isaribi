mod kagura {
    pub use kagura::prelude::*;
}

pub struct State {}

pub enum Msg {}

impl State {
    pub fn new() -> Self {
        Self {}
    }
    pub fn update(msg: Msg) {}
}

pub fn frame<M>(
    state: State,
    messenger: impl Fn() -> crate::MessengerGen<Msg, M>,
    children: Vec<kagura::Html<M>>,
) -> kagura::Html<M> {
    kagura::Html::div(kagura::Attributes::new(), kagura::Events::new(), vec![])
}
