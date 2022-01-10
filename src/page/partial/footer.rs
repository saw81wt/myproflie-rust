use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    footer![
        C![
            "items-center",
            "p-4",
            "footer",
            "bg-base-content",
            "text-neutral-content",
        ],
        div![],
    ]
}
