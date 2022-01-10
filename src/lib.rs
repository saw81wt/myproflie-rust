#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

struct Model {}

enum Msg {}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {}

fn view(_: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![
            C!["top-1"],
            "Hello World"
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::start("app", init, update, view);
}
