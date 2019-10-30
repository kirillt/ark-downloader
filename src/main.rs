use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

fn submit_link(req: Request<Body>) -> Response<Body> {
    Response::new(Body::from("Not implemented yet\n"))
}

fn main() {
    let addr = ([0, 0, 0, 0], 1337).into();

    let service = || {
        service_fn_ok(submit_link)
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Starting the server at {:?}", addr);

    hyper::rt::run(server);
}
