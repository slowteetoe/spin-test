use anyhow::Result;

use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn handle_hello_rust(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let res = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("GET")
            .uri("http://172.30.1.16") // helloworld running on my k8s cluster
            .body(None)?,
    )?;

    let response = format!("Hello, Proxy\n\n{:?}", res);

    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some(response.into()))?)
}
