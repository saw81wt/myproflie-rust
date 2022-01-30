use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![
            "text-center",
        ],
        p!["not found"]
    ]
}
