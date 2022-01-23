use crate::{Msg, Model};
use seed::{prelude::*, *};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C![
            "text-center",
            "hero-content"
        ],
        div![
            C!["max-w-md"],
            canvas![
                el_ref(&model.canvas),
                attrs![
                    At::Width => px(model.mycanvas.width),
                    At::Height => px(model.mycanvas.height)
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
                    St::Width => format!("{}px", model.mycanvas.view_width),
                    St::Height => format!("{}px", model.mycanvas.view_height)
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

