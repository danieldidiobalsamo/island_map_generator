use generator::{self, Generator, GeneratorSettings};
use log::info;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use web_sys::HtmlElement;

enum Msg {
    Generate,
}

struct Model {
    map_width: f64,
    map_height: f64,
    generator: Generator,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            map_width: 500.0,
            map_height: 800.0,
            generator: Generator::new(GeneratorSettings::new(
                6,
                1.0,
                0.5,
                1.0,
                2.0,
                (200.0, 200.0),
                0.5,
                101,
            )),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                let document = web_sys::window()
                    .expect("can't get window")
                    .document()
                    .expect("can't get document");

                let canvas = document
                    .get_element_by_id("islandCanvas")
                    .expect("canvas not found");
                let canvas: web_sys::HtmlCanvasElement = canvas
                    .dyn_into::<web_sys::HtmlCanvasElement>()
                    .map_err(|_| ())
                    .unwrap();

                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<web_sys::CanvasRenderingContext2d>()
                    .unwrap();

                context.clear_rect(0.0, 0.0, self.map_width, self.map_width);

                for x in 0..self.map_width as u64 {
                    for y in 0..self.map_height as u64 {
                        let color = self.generator.get_pixel_color((x, y));
                        let rgb = format!("rgb({},{},{})", color.0, color.1, color.2);

                        context.set_fill_style(&rgb.into());
                        context.fill_rect(x as f64, y as f64, 1.0, 1.0);
                    }
                }

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {

            <body>

                <canvas id="islandCanvas" width=500 height=800></canvas>

                <div id="settings" style="width: 100px;display: inline-block;">
                  <label for="octaves">{"octaves"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="octaves"/>

                  <label for="amplitude">{"amplitude"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="amplitude"/>

                  <label for="frequency">{"frequency"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="frequency"/>

                  <label for="persistence">{"persistence"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="persistence"/>

                  <label for="lacunarity">{"lacunarity"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="lacunarity"/>

                  <label for="scale">{"scale"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="scale"/>

                  <label for="bias">{"bias"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="bias"/>

                  <label for="seed">{"seed"}</label>
                  <input type="range" min="1" max="100" value="50" class="slider" id="seed"/>

                  <button id="generate_btn" onclick={link.callback(|_| Msg::Generate)}>{"Generate"}</button>

                </div>

            </body>
        }
    }
}

fn main() {
    console_log::init_with_level(Level::Debug);
    yew::start_app::<Model>();
}
