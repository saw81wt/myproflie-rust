use crate::{Msg, Mnist};
use seed::{prelude::{*, web_sys::HtmlCanvasElement}, *};

pub fn view(canvas: &ElRef<HtmlCanvasElement>, _: &Mnist) -> Node<Msg> {
    div![
        C![
            "text-center",
            "hero-content"
        ],
        div![
            C!["max-w-md"],
            canvas![
                el_ref(canvas),
                attrs![
                    At::Width => px(28),
                    At::Height => px(28)
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
                    St::Width => "400px",
                    St::Height => "400px"
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

