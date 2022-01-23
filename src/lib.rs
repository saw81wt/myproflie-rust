#![allow(clippy::wildcard_imports)]

mod page;
use seed::{prelude::{*, web_sys::{HtmlCanvasElement}}, *};

const TITLE_SUFFIX: &str = "SotaroProfile";
const IMAGES_PATH: &str = "static/images";
const ABOUT: &str = "about";
const MNIST: &str = "mnist";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    orders.after_next_render(|_| Msg::Rendered);

    Model {
        base_url: url.to_base_url(),
        page: Page::Home,
        mnist: Default::default(),
        canvas: Default::default(),
        drawable: false,
    }
}

#[derive(Default)]
pub struct Model {
    base_url: Url,
    pub page: Page,
    pub mnist: Mnist,
    pub canvas: ElRef<HtmlCanvasElement>,
    drawable: bool,
}


#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    MNIST,
    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {TITLE_SUFFIX}")),
            [MNIST] => (Self::MNIST, format!("Mnist - {TITLE_SUFFIX}")),
            _ => (Self::NotFound, format!("Not found - {TITLE_SUFFIX}"))
        };
        document().set_title(&title);
        page
    }
}

impl Default for Page {
    fn default() -> Self {
        Page::Home
    }
}

pub struct Mnist {
    user_input: Vec<u8>,
    estimate_number: Option<u8>
}

impl Default for Mnist {
    fn default() -> Self {
        Self {
            user_input: vec![],
            estimate_number: None
        }
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Rendered,
    PointerDown(web_sys::MouseEvent),
    PointerUp(web_sys::MouseEvent),
    PointerMove(web_sys::MouseEvent),
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }

    pub fn mnist(self) -> Url {
        self.base_url().add_path_part(MNIST)
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
        Msg::Rendered => {
            orders.after_next_render(|_| Msg::Rendered).skip();
        },
        Msg::PointerDown(mouse_event) => {
            let canvas = model.canvas.get().expect("get canvas");
            let ctx = seed::canvas_context_2d(&canvas);
            ctx.begin_path();
            ctx.move_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
            model.drawable = true;
        },
        Msg::PointerMove(mouse_event) => {
            if model.drawable {
                let canvas = model.canvas.get().expect("get canvas");
                let ctx = seed::canvas_context_2d(&canvas);
                ctx.line_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
                ctx.stroke();
                ctx.begin_path();
                ctx.move_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
            }
        },
        Msg::PointerUp(mouse_event) => {
            let canvas = model.canvas.get().expect("get canvas");
            let ctx = seed::canvas_context_2d(&canvas);
            ctx.line_to(mouse_event.offset_x() as f64, mouse_event.offset_y() as f64);
            ctx.stroke();
            model.drawable = false;
        },
    }
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![
            C![
                "flex",
                "flex-col",
                "min-h-screen"
            ],
            page::partial::header::view(&model),
            match model.page {
                Page::Home => page::home::view(),
                Page::About => page::about::view(),
                Page::MNIST => page::mnist::view(&model.canvas, &model.mnist),
                Page::NotFound => page::not_found::view(),
            },
            page::partial::footer::view(),
        ]
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{IMAGES_PATH}/{image}")
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::start("app", init, update, view);
}
