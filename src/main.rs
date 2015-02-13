extern crate iron;
extern crate router;
extern crate jsonway;
extern crate "rustc-serialize" as rustc_serialize;

use iron::{Request, Response};
use iron::status;
use rustc_serialize::json::ToJson;
use jsonway::*;

use quokka::Application;
mod quokka;

pub struct App {
    name: String
}

pub struct AppSerializer;
impl jsonway::ObjectSerializer<App> for AppSerializer {
    fn root(&self) -> Option<&str> { Some("app") }

    fn build(&self, app: &App, json: &mut jsonway::ObjectBuilder) {
        json.set("name", app.name.to_string());
    }
}

fn main() {
    let mut app = Application::new();

    app.get("/api/apps", |&: _: &mut Request| {
        let app = App { name: "Happy Numbers".to_string() };
        Ok(Response::with((status::Ok, AppSerializer.serialize(&app, true).to_string())))
    });

    quokka::run(app);
}
