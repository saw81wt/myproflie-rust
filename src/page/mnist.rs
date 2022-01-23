use crate::{Msg, Mnist};
use seed::{prelude::{*, web_sys::HtmlCanvasElement}, *};

pub fn view(canvas: &ElRef<HtmlCanvasElement>, _: &Mnist) -> Node<Msg> {
    div![
        C![
            "bg-base-200",
            "flex-grow"
        ],
        canvas![
            el_ref(canvas),
            attrs![
                At::Width => px(400),
                At::Height => px(400)
            ],
            ev(Ev::PointerDown, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::PointerDown(mouse_event)
            }),
            ev(Ev::PointerUp, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::DrawEnd(mouse_event)
            }),
            ev(Ev::PointerLeave, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::DrawEnd(mouse_event) 
            }),
            ev(Ev::PointerMove, |event| {
                let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                Msg::PointerMove(mouse_event)
            }),
            style![
                St::Border => "5px solid black"
            ]
        ]
    ]
}

