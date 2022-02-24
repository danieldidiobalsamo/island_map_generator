use island_map_generator::Model;
use log::info;
use log::Level;

fn main() {
    console_log::init_with_level(Level::Debug);
    yew::start_app::<Model>();
}
