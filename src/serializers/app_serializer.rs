use rustc_serialize::json::ToJson;
use jsonway::{ObjectBuilder, ObjectSerializer};

use models::app;

pub struct AppSerializer;
impl ObjectSerializer<app::App> for AppSerializer {
    fn root(&self) -> Option<&str> { Some("app") }

    fn build(&self, app: &app::App, json: &mut ObjectBuilder) {
        json.set("name", app.name.to_string());
    }
}
