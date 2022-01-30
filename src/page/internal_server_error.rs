use crate::{Msg, Model, Urls};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "text-center",
        ],
        div![
            C!["max-w-md"],
            p![
                C!["mb-5"],
                "すみません。問題がおきてしまったようです。"
            ],
            a![
                C!["mb-5"],
                attrs! {
                    At::Href => Urls::new(&model.base_url).home()
                },
                "TOPページへ戻る"
            ]
        ]
    ]
}
