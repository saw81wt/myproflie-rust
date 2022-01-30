use crate::{Msg, Model, Urls};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "sidebar bg-neutral w-48 space-y-6 py-7 px-2 absolute inset-y-0 left-0",
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
                    "Home",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).home()
                    },
                ]
            ],
            li![
                a![
                    "About",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).about()
                    },
                ]
            ],
            li![
                a![
                    "MNIST",
                    attrs! {
                        At::Href => Urls::new(&model.base_url).mnist()
                    },
                ]
            ]
        ],
    ]
}
