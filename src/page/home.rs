use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![
            "text-center",
            "hero-content",
        ],
        div![
            C!["max-w-md"],
            h1![
                C![
                    "mb-5",
                    "text-5xl",
                    "font-bold"
                ],
                "Welcome!"
            ],
            p![
                C!["md-5"],
                "This site was created by ",
                a![
                    C!["link", "link-neutral"],
                    attrs! {
                        At::Href => "https://seed-rs.org/"
                    },
                    "Seed"
                ],
                ", a front-end Rust framework with an elm-like architecture."
            ]
        ],
    ]
}
