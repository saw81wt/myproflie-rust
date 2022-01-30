#![allow(clippy::wildcard_imports)]

mod page;
mod config;
use seed::{C, div, nodes, document, window};
#[cfg(debug_assertions)]
use seed::console_error_panic_hook;
use seed::prelude::*;
use tract_onnx::prelude::*;
use image;
use base64;
use std::io::BufReader;

const TITLE_SUFFIX: &str = "SotaroProfile";
const IMAGES_PATH: &str = "static/images";
const ABOUT: &str = "about";
const MNIST: &str = "mnist";
const INTERNAL_SERVER_ERROR: &str = "internal_server_error";

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged)
        .send_msg(Msg::WindowResized)
        .after_next_render(|_| Msg::Rendered)
        .stream(streams::window_event(Ev::Resize, |_| Msg::WindowResized));

    Model {
        base_url: url.to_base_url(),
        page: Page::Home,
        canvas: Default::default(),
        var_hidden: true,
        canvas_settings: CanvasSettings {
            height: config::CANVAS_HEIGHT,
            width: config::CANVAS_WIDTH,
            view_height: config::CANVAS_VIEW_MIN_HEIGHT,
            view_width: config::CANVAS_VIEW_MIN_WIDTH,
            line_width: config::CANVAS_LINE_WIDTH,
            line_cap: config::ACNVAS_LINE_CAP
        },
        drawable: false,
        estimate_number: None
    }
}

pub struct Model {
    base_url: Url,
    page: Page,
    canvas: ElRef<web_sys::HtmlCanvasElement>,
    var_hidden: bool,
    canvas_settings: CanvasSettings,
    estimate_number: Option<u8>,
    drawable: bool,
}


#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Page {
    Home,
    About,
    MNIST,
    NotFound,
    InternalSeverError,
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        let (page, title) = match url.remaining_path_parts().as_slice() {
            [] => (Self::Home, TITLE_SUFFIX.to_owned()),
            [ABOUT] => (Self::About, format!("About - {TITLE_SUFFIX}")),
            [MNIST] => (Self::MNIST, format!("Mnist - {TITLE_SUFFIX}")),
            [INTERNAL_SERVER_ERROR] => (Self::InternalSeverError, TITLE_SUFFIX.to_owned()),
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

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct CanvasSettings {
    height: u32,
    width: u32,
    view_height: u32,
    view_width: u32,
    line_width: u8,
    line_cap: &'static str
}

impl CanvasSettings {
    pub fn convert_offset_x_to_draw_point_x(self: Self, x: f64) -> f64 {
        match self.view_width {
            0_u32 => 0.0,
            _ => x * self.width as f64 / self.view_width as f64
        }
    }

    pub fn convert_offset_y_to_draw_point_y(self: Self, y: f64) -> f64 {
        match self.view_height {
            0_u32 => 0.0,
            _ => y * self.height as f64 / self.view_height as f64,
        }
    }
}

pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Rendered,
    DrawStart(web_sys::MouseEvent),
    Drawing(web_sys::MouseEvent),
    DrawEnd(web_sys::MouseEvent),
    ClearCanvas,
    TranslateSlideBar,
    CloseSlideBar,
    WindowResized,
}

seed::struct_urls!();
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

    pub fn internal_server_error(self) -> Url {
        self.base_url().add_path_part(INTERNAL_SERVER_ERROR)
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
            model.var_hidden = true;
        },
        Msg::Rendered => {
            orders.after_next_render(|_| Msg::Rendered).skip();
        },
        Msg::DrawStart(mouse_event) => {
            let canvas = model.canvas.get().expect("get canvas");
            let ctx = seed::canvas_context_2d(&canvas);
            ctx.set_line_width(model.canvas_settings.line_width as f64);
            ctx.set_line_cap(model.canvas_settings.line_cap);
            ctx.begin_path();
            ctx.move_to(
                model.canvas_settings.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                model.canvas_settings.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
            );
            model.drawable = true;
        },
        Msg::Drawing(mouse_event) => {
            if model.drawable {
                let canvas = model.canvas.get().expect("get canvas");
                let ctx = seed::canvas_context_2d(&canvas);
                ctx.line_to(
                    model.canvas_settings.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                    model.canvas_settings.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
                );
                ctx.stroke();
                ctx.set_line_width(model.canvas_settings.line_width as f64);
                ctx.set_line_cap(model.canvas_settings.line_cap);
                ctx.begin_path();
                ctx.move_to(
                    model.canvas_settings.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                    model.canvas_settings.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
                );
            }
        },
        Msg::DrawEnd(mouse_event) => {
            if model.drawable {
                let canvas = model.canvas.get().expect("get canvas");
                let ctx = seed::canvas_context_2d(&canvas);
                ctx.line_to(
                    model.canvas_settings.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                    model.canvas_settings.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
                );
                ctx.stroke();
                model.drawable = false;
                
                
                let buffer: Vec<u8> = match canvas.to_data_url_with_type("image/png") {
                    Ok(image_str) => { 
                        let image_str = image_str.to_string().replace("data:image/png;base64,", "");
                        base64::decode(&image_str).unwrap()
                    },
                    Err(_) => { orders.request_url(Urls::new(&model.base_url).internal_server_error()); return; },
                };

                let input_data = match image::load_from_memory_with_format(&buffer, image::ImageFormat::Png) {
                    Ok(input_data) => input_data.to_luma_alpha8(),
                    Err(_) => { orders.request_url(Urls::new(&model.base_url).internal_server_error()); return; },
                };

                match predict(&input_data) {
                    Ok(result) => {
                        if let Some(estimate_number) = result {
                            model.estimate_number = Some(estimate_number.1 as u8);
                        }
                    },
                    Err(_) => { orders.request_url(Urls::new(&model.base_url).internal_server_error()); return; },
                }
            }
        },
        Msg::ClearCanvas => {
            let canvas = model.canvas.get().expect("get canvas");
            let ctx = seed::canvas_context_2d(&canvas);
            ctx.clear_rect(0.0, 0.0, model.canvas_settings.width as f64, model.canvas_settings.height as f64)
        },
        Msg::TranslateSlideBar => {
            model.var_hidden = !model.var_hidden;
        },
        Msg::CloseSlideBar => {
            model.var_hidden = true;
        },
        Msg::WindowResized => {
            let window = window();
            let width = window
                .inner_width()
                .expect("window width")
                .unchecked_into::<js_sys::Number>()
                .value_of();
            let height = window
                .inner_height()
                .expect("window height")
                .unchecked_into::<js_sys::Number>()
                .value_of();
            if width > config::TAILWIND_MD_WIDTH {
                model.var_hidden = true
            }
            let min = std::cmp::min(height as u32, width as u32);
            model.canvas_settings.view_height = std::cmp::min(min, config::CANVAS_VIEW_MIN_HEIGHT);
            model.canvas_settings.view_width = std::cmp::min(min, config::CANVAS_VIEW_MIN_WIDTH);
        }
    }
}

fn predict(input_data: &image::ImageBuffer<image::LumaA<u8>, Vec<u8>>) ->TractResult<Option<(f32, i32)>> {
    let model_byte = include_bytes!(r#"../static/model/mnist-8.onnx"#);
    let onxx_model: SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>> = tract_onnx::onnx()
        .model_for_read(&mut BufReader::new(&model_byte[..]))?
        .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 1, 28, 28)))?
        .into_optimized()?
        .into_runnable()?;
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 1, 28, 28), |(_, _, y, x)| -> f32 {
        input_data[(x as _, y as _)][1] as f32 / 255.0
    }).into();

    let result = onxx_model.run(tvec![image])?;
    let best: Option<(f32, i32)> = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(0..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
    Ok(best)
}

fn view(model: &Model) -> Vec<seed::virtual_dom::Node<Msg>> {
    nodes![
        div![
            C![
                "flex",
                "flex-col",
                "min-h-screen"
            ],
            page::partial::header::view(&model),
            div![
                C![
                    "hero",
                    "bg-base-200",
                    "flex-1"
                ],
                page::partial::slide_var::view(&model),
                match model.page {
                    Page::Home => page::home::view(),
                    Page::About => page::about::view(),
                    Page::MNIST => page::mnist::view(&model),
                    Page::NotFound => page::not_found::view(),
                    Page::InternalSeverError => page::internal_server_error::view(&model),
                },
            ],
        ]
    ]
}

pub fn image_src(image: &str) -> String {
    format!("{IMAGES_PATH}/{image}")
}

#[wasm_bindgen(start)]
pub fn start() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    App::start("app", init, update, view);
}
