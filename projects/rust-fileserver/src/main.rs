extern crate hyper;

use hyper::{Body, Request, Response, Server, StatusCode, Method};
use hyper::rt::{Future};
use hyper::service::service_fn;
use futures::future;

use std::io::prelude::*;
use std::fs::File;

// In english: Allocated a dynamic future on the heap where the
// item is a response with a body, and the erro is produced by hyper
// Note: don't know what the '+ Send' means yet...
type BoxedFuture = Box<dyn Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn serve_music_file(res: &mut Response<Body>, filename: String) {
    let mut file_path: String = String::from("testfiles/");
    file_path.push_str(&filename);

    let mut file = File::open(&file_path).unwrap();
    let mut buffer: Vec<u8> = vec![];
    match file.read_to_end(&mut buffer) {
        Ok(_) => {
            match(file_path) {
                ref path if path.contains(".wav") => {
                    res.headers_mut().insert("content-type", hyper::http::HeaderValue::from_static("audio/wave"));
                },
                ref path if path.contains(".mp3") => {
                    res.headers_mut().insert("content-type", hyper::http::HeaderValue::from_static("audio/mp3"));
                },
                _ => {
                    res.headers_mut().insert("content-type", hyper::http::HeaderValue::from_static("application/octet-stream"));
                }
            }
            *res.body_mut() = Body::from(buffer);
            *res.status_mut() = StatusCode::OK;
        },
        Err(_) => *res.body_mut() = Body::from("Unable to get file")
    }
}

fn serve_file(req: Request<Body>) -> BoxedFuture {
    let mut res = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::GET, wav_path) if wav_path.contains("/wav/") => {
            serve_music_file(&mut res, wav_path.trim_start_matches("/wav/").to_string());
        },
        (&Method::GET, mp3_path) if mp3_path.contains("/mp3/") => {
            serve_music_file(&mut res, mp3_path.trim_start_matches("/mp3/").to_string());
        },
        _ => {
            *res.status_mut() = StatusCode::NOT_FOUND;
        }
    }

    Box::new(future::ok(res))
}



fn main() {
    println!("Hello, world!");
    let addr = ([127, 0, 0, 1], 3000).into();
    let server = Server::bind(&addr)
        .serve(|| {
            service_fn(serve_file)
        })
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on: http://{}", addr);
    hyper::rt::run(server);
}
