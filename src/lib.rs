use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {}
}

struct Model {

}

enum Msg {

}

fn update(_: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {

}

fn view(_: &Model) -> Vec<Node<Msg>> {
    nodes![
        header![C![""]]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let root_element = document()
        .get_elements_by_class_name("todoapp")
        .item(0)
        .expect("element with the class `todoapp`");

    App::start(root_element, init, update, view);
}
