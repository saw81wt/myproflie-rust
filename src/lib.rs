#![allow(clippy::wildcard_imports)]

mod page;
use seed::{prelude::*, *};

const TITLE_SUFFIX: &str = "SotaroProfile";
const IMAGES_PATH: &str = "static/images";
const ABOUT: &str = "about";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);

    Model {
        base_url: url.to_base_url(),
        page: Page::Home
    }
}

pub struct Model {
    base_url: Url,
    pub page: Page,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    NotFound,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {}", TITLE_SUFFIX)),
            _ => (Self::NotFound, format!("Not found - {}", TITLE_SUFFIX))
        };
        document().set_title(&title);
        page
    }
}
pub enum Msg {
    UrlChanged(subs::UrlChanged),
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }

    pub fn about(self) -> Url {
        self.base_url().add_path_part(ABOUT)
    }
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        }
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
                Page::NotFound => page::not_found::view(),
            },
            page::partial::footer::view(),
        ]
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{}/{}", IMAGES_PATH, image)
}

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::start("app", init, update, view);
}
