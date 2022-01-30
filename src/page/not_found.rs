use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![
            "hero-content",
            "text-center",
        ],
        p!["not found"]
    ]
}
