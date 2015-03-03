extern crate iron;
extern crate router;
extern crate jsonway;
extern crate "rustc-serialize" as rustc_serialize;

use quokka::Application;
use serializers::app_serializer;
use models::app;
use jsonway::serializer::ObjectSerializer;

mod quokka;
mod serializers;
mod models;

fn main() {
    let mut app = Application::new();

    app.get("/api/apps", |req, res| {
        let app = app::App { name: "Happy Numbers".to_string() };
        res.send(app_serializer::AppSerializer.serialize(&app, true).to_string())
    });

    quokka::run(app);
}
