extern crate isaribi;
extern crate kagura;
extern crate wasm_bindgen;

use kagura::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    kagura::run(Component::new(init, update, render), "app");
}

struct State {}

enum Msg {
    NoOp,
}

enum Sub {}

fn init() -> (State, Cmd<Msg, Sub>) {
    (State {}, Cmd::none())
}

fn update(state: &mut State, msg: Msg) -> Cmd<Msg, Sub> {
    Cmd::none()
}

fn render(state: &State) -> Html<Msg> {
    isaribi::frame(isaribi::frame::State::new(), |_| Msg::NoOp, vec![])
}
