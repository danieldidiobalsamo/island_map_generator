use island_map_generator_algo::{self, Generator};
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
    fn get_generation_setting_as_string(
        &mut self,
        name: &str,
        document: &web_sys::Document,
    ) -> String {
        document
            .get_element_by_id(name)
            .unwrap()
            .dyn_ref::<HtmlInputElement>()
            .unwrap()
            .value()
    }

    fn update_generator_settings(&mut self, document: &web_sys::Document) {
        let octaves = self
            .get_generation_setting_as_string("octaves", &document)
            .parse::<usize>()
            .unwrap();

        let frequency = self
            .get_generation_setting_as_string("frequency", &document)
            .parse::<f64>()
            .unwrap();

        let persistence = self
            .get_generation_setting_as_string("persistence", &document)
            .parse::<f64>()
            .unwrap();

        let lacunarity = self
            .get_generation_setting_as_string("lacunarity", &document)
            .parse::<f64>()
            .unwrap();

        let scale = self
            .get_generation_setting_as_string("scale", &document)
            .parse::<f64>()
            .unwrap();

        let bias = self
            .get_generation_setting_as_string("bias", &document)
            .parse::<f64>()
            .unwrap();

        let seed = self
            .get_generation_setting_as_string("seed", &document)
            .parse::<u32>()
            .unwrap();

        self.generator = Generator::new(
            octaves,
            frequency,
            persistence,
            lacunarity,
            scale,
            bias,
            seed,
        );
    }

    fn generate_map(&mut self) {
        let document = web_sys::window()
            .expect("can't get window")
            .document()
            .expect("can't get document");

        self.update_generator_settings(&document);

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
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            map_width: 300.0,
            map_height: 400.0,
            generator: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Generate => {
                self.generate_map();

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {

            <body>

                <canvas id="islandCanvas" width=300 height=400></canvas>

                <div id="settings" style="width: 100px;display: inline-block;">
                  <label for="octaves">{"octaves"}</label>
                  <input type="range" min="1" max="10" value="6" class="slider" id="octaves"/>

                  <label for="frequency">{"frequency"}</label>
                  <input type="range" min="1" max="10" step="0.1" value="4.2" class="slider" id="frequency"/>

                  <label for="persistence">{"persistence"}</label>
                  <input type="range" min="1" max="2" value="1.5" step="0.05" class="slider" id="persistence"/>

                  <label for="lacunarity">{"lacunarity"}</label>
                  <input type="range" min="1" max="1.5" value="1.2" step="0.05" class="slider" id="lacunarity"/>

                  <label for="scale">{"scale"}</label>
                  <input type="range" min="0" max="2" value="1" step="0.01" class="slider" id="scale"/>

                  <label for="bias">{"bias"}</label>
                  <input type="range" min="-0.5" max="0.5" value="0.17" step="0.01" class="slider" id="bias"/>

                  <label for="seed">{"seed"}</label>
                  <input type="number" min="0" max={u32::MAX.to_string()} value="9000" id="seed"/>

                  <button id="generate_btn" onclick={link.callback(|_| Msg::Generate)}>{"Generate"}</button>

                </div>

            </body>
        }
    }
}
