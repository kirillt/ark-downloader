mod job;

use hyper::{Body, Request, Response, Method, StatusCode, Server};
use hyper::rt::Future;
use hyper::service::service_fn;

use std::sync::{Arc, Mutex};
use evmap::{ReadHandle, WriteHandle};

use futures::{future, Stream};

use job::Job;

#[derive(Clone)]
struct Downloader {
    jobs_w: Arc<Mutex<WriteHandle<u64, Job>>>,
    jobs_r: ReadHandle<u64, Job>
}

type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn ok(value: Response<Body>) -> ResponseFuture {
    Box::new(future::ok(value))
}

impl Downloader {
    fn submit_link(&self, request: Request<Body>) -> ResponseFuture {
        match (request.method(), request.uri().path()) {
            (&Method::GET, "/") => {
                ok(Response::new(Body::from("Try POSTing data to /submit\n")))
            },
            (&Method::POST, "/submit") => {
                let state = self.clone();
                Box::new(request
                    .into_body()
                    .concat2()
                    .map(move |chunk| {
                        let data: Vec<u8> = chunk.into_iter().collect();
                        let id = seahash::hash(&data[..]);
                        if state.jobs_r.contains_key(&id) {
                            Response::new(Body::from(
                                format!("The data `{}` is already submitted.\n", id)))
                        } else {
                            let mut jobs = state.jobs_w.lock().unwrap();
                            jobs.insert(id, Job { id });
                            jobs.refresh();
                            Response::new(Body::from(
                                format!("Scheduling download of `{:?}` with id {}.\n",
                                        String::from_utf8(data), id)))
                        }
                    }))
            },
            _ => {
                ok(Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap())
            },
        }
    }
}

fn main() {
    let addr = ([0, 0, 0, 0], 1337).into();

    let (jobs_r, jobs_w) = evmap::new();

    let state = Downloader {
        jobs_w: Arc::new(Mutex::new(jobs_w)),
        jobs_r
    };

    let service = move || {
        println!("Creating new state");
        let state = state.clone();
        service_fn(move |request| state.submit_link(request))
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Starting the server at {:?}", addr);

    hyper::rt::run(server);
}
