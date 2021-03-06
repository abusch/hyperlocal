extern crate futures;
extern crate hyper;
extern crate hyperlocal;

use hyper::service::service_fn;
use hyper::{header, Body, Request, Response};
use std::io;

const PHRASE: &'static str = "It's a Unix system. I know this.";

fn hello(
    req: Request<Body>,
) -> impl futures::Future<Item = Response<Body>, Error = io::Error> + Send {
    println!("servicing new request {:?}", req);
    futures::future::ok(
        Response::builder()
            .header(header::CONTENT_TYPE, "text/plain")
            .header(header::CONTENT_LENGTH, PHRASE.len() as u64)
            .body(PHRASE.into())
            .expect("failed to create response"),
    )
}

fn run() -> io::Result<()> {
    let svr = hyperlocal::server::Http::new().bind("test.sock", || service_fn(hello))?;

    for path in svr.local_addr()
        .ok()
        .and_then(|addr| addr.as_pathname().map(|pathname| pathname.to_owned()))
    {
        println!(
            "Listening on unix://{path} with 1 thread.",
            path = path.as_path().to_string_lossy()
        );
    }
    svr.run()?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error starting server: {}", err)
    }
}
