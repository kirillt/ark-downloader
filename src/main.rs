mod job;

use hyper::{Body, Chunk, Request, Response, Method, StatusCode, Server};
use hyper::rt::Future;
use hyper::service::service_fn;

use std::sync::{Arc, Mutex};
use evmap::{ReadHandle, WriteHandle};

use futures::{future, Stream};

use clap::Clap;

use job::Job;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Options {
    #[clap(short, long, default_value = "1337")]
    port: u16,
    #[clap(short, long, default_value = ".")]
    directory: String,
}

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
        self.refresh();

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

                        let maybe = state.jobs_r
                            .get_and(&id, |jobs| {
                                assert!(jobs.len() == 1);
                                let job = &jobs[0];
                                job.status()
                            });

                        match maybe {
                            Some(status) => {
                                Response::new(Body::from(
                                    format!("The data is already {} with id {}\n", status, id)))
                            },
                            None => {
                                let resource = String::from_utf8(data).unwrap();
                                let mut jobs = state.jobs_w.lock().unwrap();
                                jobs.insert(id, Job::start(id, resource.clone()));
                                jobs.refresh();
                                Response::new(Body::from(
                                    format!("Scheduling download of `{:?}` with id {}\n",
                                            resource, id)))
                            }
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

    fn refresh(&self) {
        let mut jobs = self.jobs_w.lock().unwrap();
    }
}

fn main() {
    let options: Options = Options::parse();
    println!("Listening port {}", options.port);
    println!("Downloading eveything into folder {}", options.directory);
    if &options.directory != "." {
        std::env::set_current_dir(options.directory).unwrap();
    }

    let addr = ([0, 0, 0, 0], options.port).into();

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
