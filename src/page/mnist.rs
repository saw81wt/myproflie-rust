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
                    At::Width => px(400),
                    At::Height => px(400)
                ],
                ev(Ev::PointerDown, |event| {
                    let mouse_event: web_sys::MouseEvent = match event.dyn_into() {
                        Ok(mouse_event) => mouse_event,
                        Err(error) => panic!("failed to cast mouse event. {:?}", error)
                    };
                    Msg::PointerDown(mouse_event)
                }),
                ev(Ev::PointerUp, |event| {
                    let mouse_event: web_sys::MouseEvent = match event.dyn_into() {
                        Ok(mouse_event) => mouse_event,
                        Err(error) => panic!("failed to cast mouse event. {:?}", error)
                    };
                    Msg::DrawEnd(mouse_event)
                }),
                ev(Ev::PointerLeave, |event| {
                    let mouse_event: web_sys::MouseEvent = match event.dyn_into() {
                        Ok(mouse_event) => mouse_event,
                        Err(error) => panic!("failed to cast mouse event. {:?}", error)
                    };
                    Msg::DrawEnd(mouse_event) 
                }),
                ev(Ev::PointerMove, |event| {
                    let mouse_event: web_sys::MouseEvent = match event.dyn_into() {
                        Ok(mouse_event) => mouse_event,
                        Err(error) => panic!("failed to cast mouse event. {:?}", error)
                    };
                    Msg::PointerMove(mouse_event)
                }),
                style![
                    St::Border => "5px solid black"
                ]
            ],
        ]   
    ]
}

