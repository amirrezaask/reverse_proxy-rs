use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use std::convert::From;
use std::convert::Infallible;
use std::net::SocketAddr;

fn get_dst_host(req: &Request<Body>) -> String {
    let _dst = req.uri().path_and_query().unwrap().path();
    let dst = _dst.to_owned();
    // dst.split_at(1).1.to_owned()
    format!("{}", "http://httpbin.org/anything")
}

async fn pass(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = Client::new();
    let uri = get_dst_host(&_req).parse().unwrap();
    let (parts, body) = _req.into_parts();
    let mut req = Request::new(body);
    *req.method_mut() = parts.method;
    *req.uri_mut() = uri;
    *req.headers_mut() = parts.headers;
    let resp = client.request(req).await.unwrap();
    Ok(resp)
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(pass)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
