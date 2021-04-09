use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, Uri};
use std::convert::From;
use std::convert::Infallible;
use std::net::SocketAddr;

fn get_dst_host(req: &Request<Body>) -> String {
    let _dst = req.uri().path_and_query().unwrap().path();
    let dst = _dst.to_owned();
    // dst.split_at(1).1.to_owned()
    format!("{}{}", "https://httpbin.org", dst)
}

async fn pass(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let client = Client::new();
    println!("{}", get_dst_host(&_req));
    let mut req = Request::new(Body::default());
    *req.uri_mut() = get_dst_host(&_req).parse().unwrap();
    let resp = client.request(req).await.unwrap_or_default();
    println!("{:?}", resp);
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
