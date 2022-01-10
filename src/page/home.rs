use crate::Msg;
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![
            "hero",
            "bg-base-200",
            "flex-grow"
        ],
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
                ]
            ]
        ],
    ]
}
