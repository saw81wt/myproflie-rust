use crate::{Msg, Model};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "text-center",
        ],
        div![
            p![
                C![
                    "text-x1",
                    "mb-5",
                ],
                if let Some(estimate_number) = model.estimate_number{
                    format!("あなたが書いた数字は{estimate_number}です").to_string()
                }
                else {
                    "0~9までの数字を書いてください".to_string()
                }
            ],
            C!["max-w-md"],
            canvas![
                el_ref(&model.canvas),
                attrs![
                    At::Width => px(model.canvas_settings.width),
                    At::Height => px(model.canvas_settings.height)
                ],
                ev(Ev::PointerDown, |event| {
                    let mouse_event: web_sys::MouseEvent = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                    Msg::DrawStart(mouse_event)
                }),
                ev(Ev::PointerUp, |event| {
                    let mouse_event: web_sys::MouseEvent = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                    Msg::DrawEnd(mouse_event)
                }),
                ev(Ev::PointerLeave, |event| {
                    let mouse_event: web_sys::MouseEvent = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                    Msg::DrawEnd(mouse_event) 
                }),
                ev(Ev::PointerMove, |event| {
                    let mouse_event: web_sys::MouseEvent = event.dyn_into::<web_sys::MouseEvent>().unwrap();
                    Msg::Drawing(mouse_event)
                }),
                style![
                    St::Border => "5px solid black",
                    St::Width => format!("{}px", model.canvas_settings.view_width),
                    St::Height => format!("{}px", model.canvas_settings.view_height)
                ],
                C!["mb-5"],
            ],
            button![
                C![
                    "btn",
                    "btn-ghost",
                    "btn-sm",
                    "rounded-btn"
                ],
                "Clear",
                ev(Ev::Click, |_| Msg::ClearCanvas)
            ]
        ]   
    ]
}

