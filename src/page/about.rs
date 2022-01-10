use crate::{Msg, image_src};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    div![
        C![
            "hero"
            "bg-base-200",
            "flex-grow"
        ],
        div![
            C![
                "hero-content",
                "lg:flex-row-reverse"
            ],
            img![
                C![
                    "max-w-sm",
                    "rounded-lg",
                    "shadow-2xl"
                ],
                attrs! {
                    At::Src => image_src("photo.jpg")
                }
            ],
            div![
                h1![
                    C![
                        "mb-5",
                        "text-5xl",
                        "font-bold",
                    ],
                    "About me",
                ],
                p![
                    C![
                        "mb-5",
                        "text-2xl"
                    ],
                    "Sotaro Maehara",
                ],
                p![
                    C![
                        "mb-5",
                        "text-opacity-40"
                    ],
                    "1994/04/07",
                ],
                p![
                    C!["mb-5"],
                    "都内で広告配信エンジニアをしています．",
                    "興味のある分野はアルゴリズム・機械学習関連．",
                    "普段触っている言語はPython/C++あたり．",
                    "最近Rustの勉強もしており，練習を兼ねてポートフォリオを作成"
                ],
                a![
                    C!["md-5"],
                    "More..."
                ],
            ],
        ],
    ]
}