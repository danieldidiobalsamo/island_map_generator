use generator::{self, Generator, GeneratorSettings};
use log::info;
use log::Level;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use web_sys::HtmlInputElement;

pub struct Model {
    map_width: f64,
    map_height: f64,
    generator: Generator,
}

pub enum Msg {
    Generate,
}

impl Model {
    fn update_generator_settings(&mut self, document: &web_sys::Document) {
        self.generator.set_frequency(
            document
                .get_element_by_id("frequency")
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<f64>()
                .unwrap(),
        );

        self.generator.set_persistence(
            document
                .get_element_by_id("persistence")
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<f64>()
                .unwrap(),
        );

        self.generator.set_lacunarity(
            document
                .get_element_by_id("lacunarity")
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<f64>()
                .unwrap(),
        );

        let scale = document
            .get_element_by_id("scale")
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
            .parse::<f64>()
            .unwrap();
        self.generator.set_scale((scale, scale));

        self.generator.set_bias(
            document
                .get_element_by_id("bias")
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<f64>()
                .unwrap(),
        );

        self.generator.set_seed(
            document
                .get_element_by_id("seed")
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .value()
                .parse::<i32>()
                .unwrap(),
        );
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            map_width: 500.0,
            map_height: 700.0,
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

                self.update_generator_settings(&document);
                self.generator.update_generator();

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

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {

            <body>

                <canvas id="islandCanvas" width=500 height=700></canvas>

                <div id="settings" style="width: 100px;display: inline-block;">
                  <label for="octaves">{"octaves"}</label>
                  <input type="range" min="1" max="50" value="6" class="slider" id="octaves"/>

                  <label for="frequency">{"frequency"}</label>
                  <input type="range" min="1" max="5" step="0.1" value="0.5" class="slider" id="frequency"/>

                  <label for="persistence">{"persistence"}</label>
                  <input type="range" min="1" max="10" value="1" step="0.1" class="slider" id="persistence"/>

                  <label for="lacunarity">{"lacunarity"}</label>
                  <input type="range" min="1" max="10" value="2" class="slider" id="lacunarity"/>

                  <label for="scale">{"scale"}</label>
                  <input type="range" min="1" max="1000" value="200" class="slider" id="scale"/>

                  <label for="bias">{"bias"}</label>
                  <input type="range" min="1" max="5" value="0.5" step="0.1" class="slider" id="bias"/>

                  <label for="seed">{"seed"}</label>
                  <input type="number" min="1" max="100000" value="0" id="seed"/>

                  <button id="generate_btn" onclick={link.callback(|_| Msg::Generate)}>{"Generate"}</button>

                </div>

            </body>
        }
    }
}
