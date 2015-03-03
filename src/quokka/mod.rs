use iron::{Iron, Request, Response, IronResult, Handler};
use iron::status;
use router::Router;

use iron::typemap::Key;

pub struct Application {
    router: Router
}

impl Application {
    pub fn new() -> Application {
        Application {
            router: Router::new()
        }
    }

    pub fn get<H: Handler, S: Str>(&mut self, path: S, handler: H) {
        self.router.get(path, handler);
    }

    pub fn listen(&self) {
    }
}

impl Handler for Application {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.router.handle(req)
    }
}

pub fn run(app: Application) {
    Iron::new(app).listen("localhost:3000").unwrap();
    println!("Quokka starting on http:://localhost:3000");
}
