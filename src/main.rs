use generator::{self, Generator, GeneratorSettings};
use std::f64;
use std::process;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use web_sys::HtmlElement;

enum Msg {
    Generate,
}

struct Model {
    mapWidth: f64,
    mapHeight: f64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            mapWidth: 1280.0,
            mapHeight: 720.0,
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

                context.clear_rect(0.0, 0.0, self.mapWidth, self.mapHeight);

                context.begin_path();

                // Draw the outer circle.
                context
                    .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
                    .unwrap();

                // Draw the mouth.
                context.move_to(110.0, 75.0);
                context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

                // Draw the left eye.
                context.move_to(65.0, 65.0);
                context
                    .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
                    .unwrap();

                // Draw the right eye.
                context.move_to(95.0, 65.0);
                context
                    .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
                    .unwrap();

                context.stroke();

                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {

            <body>

                <canvas id="islandCanvas" width=1280 height=720></canvas>

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

                  <button id="generate_btn" onclick={link.callback(|_| Msg::Generate)}>{"Generate(tmp)"}</button>

                </div>

            </body>
        }
    }
}

fn main() {
    // let generator_settings =
    //     GeneratorSettings::new(6, 1.0, 0.5, 1.0, 2.0, (200.0, 200.0), 0.5, 101, (1280, 720));

    // let generator = Generator::new(&generator_settings);
    // generator.generate().unwrap_or_else(|err| {
    //     eprintln!("Cannot write island image: {}", err);
    //     process::exit(1);
    // });

    yew::start_app::<Model>();
}
