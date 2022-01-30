use crate::{Msg, Model, Urls};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C![
                "bg-neutral max-w-full max-h-full absolute opacity-50",
                "inset-x-0 inset-y-0",
                IF!(model.var_hidden => "invisible"),
                IF!(!model.var_hidden => "visible"),
            ],
            ev(Ev::Click, |_| Msg::CloseSlideBar)
        ],
        div![
            C![
                "bg-neutral w-48 space-y-6 py-7 px-2 absolute inset-y-0 left-0 opacity-100",
                IF!(model.var_hidden => "transform -translate-x-full"),
                IF!(!model.var_hidden => "transform translate-x-0"),
                "translation duration-200 else-in-out"
            ],
            ul![
                C![
                    "menu",
                    "menu-vertical",
                    "text-base-100",
                ],
                li![
                    a![
                        C!["btn rounded"],
                        "Home",
                        attrs! {
                            At::Href => Urls::new(&model.base_url).home()
                        },
                    ]
                ],
                li![
                    a![
                        C!["btn rounded"],
                        "About",
                        attrs! {
                            At::Href => Urls::new(&model.base_url).about()
                        },
                    ]
                ],
                li![
                    a![
                        C!["btn rounded"],
                        "MNIST",
                        attrs! {
                            At::Href => Urls::new(&model.base_url).mnist()
                        },
                    ]
                ]
            ],
        ]
    ]
}
