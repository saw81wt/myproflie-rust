use crate::{Msg, image_src};
use seed::{prelude::*, *};

pub fn view() -> Node<Msg> {
    let description: &'static str = "
        都内で広告配信エンジニアをしています.
        アルゴリズム・機械学習関連に興味があります．
        普段の仕事ではPython/C++でコードを書いております．
        最近はRustの勉強もしており，練習を兼ねてポートフォリオを作成しました．
    ";
    div![
        C![
            "hero-content",
            "text-center",
        ],
        div![
            C!["max-w-md"],
            div![
                C!["mb-5"],
                span![
                    C![
                        "text-3xl"
                    ],
                    "Sotaro Maehara"
                ],
                span![
                    C![
                        "text-x1",
                        "ml-2",
                    ],
                    "1994/04/07",
                ],
                div![
                    C![
                        "avatar",
                        "p-3"
                    ],
                    img![
                        C![
                            "rounded-lg",
                            "shadow-2xl",
                            "w-24 h-24"
                        ],
                        attrs! { 
                            At::Src => image_src("photo.jpg")
                        }
                    ]
                ],
            ],
            p![
                C!["mb-5"],
                description
            ],
        ], 
    ]
}