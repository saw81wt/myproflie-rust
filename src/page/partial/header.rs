use crate::{Msg, Model, Urls};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "navbar",
            "shadow-lg",
            "bg-neutral",
            "text-neutral-content"
        ],
        div![
            C![
                "px-2",
                "mx-2",
                "navbar-start"
            ],
            span![
                C![
                    "text-lg",
                    "font-bold"
                ],
                "SotaroProfile"
            ]
        ],
        div![
            C![
                "hidden",
                "px-2",
                "mx-2",
                "navbar-center",
                "lg:flex"
            ],
            div![
                C![
                    "flex items-stretch"
                ],
                a![
                    C![
                        "btn",
                        "btn-ghost",
                        "btn-sm",
                        "rounded-btn"
                    ],
                    "Home",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).home()
                    },
                ],
                a![
                    C![
                        "btn",
                        "btn-ghost",
                        "btn-sm",
                        "rounded-btn"
                    ],
                    "About",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).about()
                    },
                ],
            ]
        ]
    ]

}
