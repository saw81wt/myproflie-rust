#![allow(clippy::wildcard_imports)]

mod page;
mod config;
use seed::{C, div, nodes, document, console_error_panic_hook};
use seed::prelude::*;
use tract_onnx::prelude::*;
use image;
use base64;
use std::io::BufReader;

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
        mycanvas: MyCanvas::init_my_canvas(),
    }
}

pub struct Model {
    base_url: Url,
    pub page: Page,
    pub mnist: Mnist,
    pub canvas: ElRef<web_sys::HtmlCanvasElement>,
    pub mycanvas: MyCanvas,
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

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct MyCanvas {
    height: u32,
    width: u32,
    view_height: u32,
    view_width: u32,
    line_width: u8,
}

impl MyCanvas {
    pub fn init_my_canvas() -> Self {
        Self {
            height: config::CANVAS_HEIGHT,
            width: config::CANVAS_WIDTH,
            view_height: config::CANVAS_VIEW_MIN_HEIGHT,
            view_width: config::CANVAS_VIEW_MIN_WIDTH,
            line_width: config::CANVAS_LINE_WIDTH
        }
    }

    pub fn convert_offset_x_to_draw_point_x(self: Self, x: f64) -> f64 {
        x * self.width as f64 / self.view_width as f64
    }

    pub fn convert_offset_y_to_draw_point_y(self: Self, y: f64) -> f64 {
        y * self.height as f64 / self.view_height as f64
    }
}

pub struct Mnist {
    // user_input: Vec<u8>,
    estimate_number: Option<u8>
}

impl Default for Mnist {
    fn default() -> Self {
        Self {
            // user_input: vec![],
            estimate_number: None
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
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page = Page::init(url);
        },
        Msg::Rendered => {
            orders.after_next_render(|_| Msg::Rendered).skip();
        },
        Msg::DrawStart(mouse_event) => {
            let canvas = model.canvas.get().expect("get canvas");
            let ctx = seed::canvas_context_2d(&canvas);
            ctx.set_line_width(model.mycanvas.line_width as f64);
            ctx.set_line_cap("round");
            ctx.begin_path();
            ctx.move_to(
                model.mycanvas.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                model.mycanvas.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
            );
            model.drawable = true;
        },
        Msg::Drawing(mouse_event) => {
            if model.drawable {
                let canvas = model.canvas.get().expect("get canvas");
                let ctx = seed::canvas_context_2d(&canvas);
                ctx.line_to(
                    model.mycanvas.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                    model.mycanvas.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
                );
                ctx.stroke();
                ctx.set_line_width(model.mycanvas.line_width as f64);
                ctx.set_line_cap("round");
                ctx.begin_path();
                ctx.move_to(
                    model.mycanvas.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                    model.mycanvas.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
                );
            }
        },
        Msg::DrawEnd(mouse_event) => {
            if model.drawable {
                let canvas = model.canvas.get().expect("get canvas");
                let ctx = seed::canvas_context_2d(&canvas);
                ctx.line_to(
                    model.mycanvas.convert_offset_x_to_draw_point_x(mouse_event.offset_x() as f64), 
                    model.mycanvas.convert_offset_y_to_draw_point_y(mouse_event.offset_y() as f64)
                );
                ctx.stroke();
                model.drawable = false;
                
                let image_str = canvas.to_data_url_with_type("image/png").unwrap();
                let image_str = image_str.to_string().replace("data:image/png;base64,", "");
                
                if let Ok(result) = predict(&image_str) {
                    if let Some(estimate_number) = result {
                        model.mnist.estimate_number = Some(estimate_number.1 as u8);
                    }
                }
            }
        },
        Msg::ClearCanvas => {
            let canvas = model.canvas.get().expect("get canvas");
            let ctx = seed::canvas_context_2d(&canvas);
            ctx.clear_rect(0.0, 0.0, model.mycanvas.width as f64, model.mycanvas.height as f64)
        }
    }
}

fn predict(image_str: &str) ->TractResult<Option<(f32, i32)>> {
    let buffer = base64::decode(&image_str).unwrap();
    let input_data = image::load_from_memory_with_format(&buffer, image::ImageFormat::Png)
        .unwrap()
        .to_luma_alpha8();

    let model_byte = include_bytes!(r#"../static/model/mnist-8.onnx"#);
    let onxx_model = tract_onnx::onnx()
        .model_for_read(&mut BufReader::new(&model_byte[..]))?
        .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), tvec!(1, 1, 28, 28)))?
        .into_optimized()?
        .into_runnable()?;
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 1, 28, 28), |(_, _, y, x)| -> f32 {
        input_data[(x as _, y as _)][1] as f32 / 255.0
    }).into();

    let result = onxx_model.run(tvec![image])?;
    let best = result[0]
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
                    "flex-grow"
                ],
                match model.page {
                    Page::Home => page::home::view(),
                    Page::About => page::about::view(),
                    Page::MNIST => page::mnist::view(&model),
                    Page::NotFound => page::not_found::view(),
                },
            ],
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
