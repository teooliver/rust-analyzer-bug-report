mod app;
mod error;
mod event_table;
mod test_component;
mod types;

fn main() {
    yew::start_app::<app::App>();
}
