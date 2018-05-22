extern crate rust_app;
extern crate yew;

use rust_app::Model;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::fetch::FetchService;

pub struct Context {
    console: ConsoleService,
    fetch: FetchService,
}

impl AsMut<ConsoleService> for Context {
    fn as_mut(&mut self) -> &mut ConsoleService {
        &mut self.console
    }
}

impl AsMut<FetchService> for Context {
    fn as_mut(&mut self) -> &mut FetchService {
        &mut self.fetch
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
        fetch: FetchService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
