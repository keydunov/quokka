use iron::{Iron, Request, Response, IronResult, Handler};
use iron::status;
use router::Router;

use iron::typemap::Key;

pub struct TestRequest;
pub struct TestResponse {
    data: Option<String>
}

impl TestResponse {
    pub fn send(&mut self, response: String) {
        self.data = Some(response);
    }
}

pub type EndpointHandler = Box<Fn(TestRequest, &mut TestResponse) + 'static + Sync>;

pub struct Application {
    router: Router,
    handler: Option<EndpointHandler>
}

unsafe impl Send for Application {}

impl Application {
    pub fn new() -> Application {
        Application {
            router: Router::new(),
            handler: None
        }
    }

    pub fn get<F: 'static, S: Str>(&mut self, path: S, handler: F)
    where F: Fn(TestRequest, &mut TestResponse) + Sync {
        self.handler = Some(Box::new(handler));
    }

    fn call(&self, req: &mut Request) -> IronResult<Response> {
        let request = TestRequest;
        let mut response = TestResponse { data: None };
        let handler = self.handler.as_ref();
        (handler.unwrap())(request, &mut response);
        match response.data {
            Some(string) => Ok(Response::with((status::Ok, string))),
            None => Ok(Response::with((status::Ok, "None".to_string()))),
        }
    }
}

impl Handler for Application {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        self.call(req)
    }
}

pub fn run(app: Application) {
    Iron::new(app).listen("localhost:3000").unwrap();
    println!("Quokka starting on http:://localhost:3000");
}
