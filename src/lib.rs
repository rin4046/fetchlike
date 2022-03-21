use hyper::{body::Buf, Client};
use hyper_tls::HttpsConnector;
use serde::de::DeserializeOwned;

pub mod error_handling;
pub mod macros;
pub mod macros2;

pub use error_handling::Error;
pub use hyper::{Body, HeaderMap, Method, Uri};

pub async fn fetch(request: Request) -> Result<Response, Error> {
    let client = Client::builder().build(HttpsConnector::new());

    let headers = request.headers;
    let mut request = hyper::Request::builder()
        .method(request.method)
        .uri(request.url)
        .body(request.body)
        .unwrap();

    if let Some(headers) = headers {
        request.headers_mut().extend(headers.clone().into_iter());
    }

    Ok(Response {
        response: client.request(request).await?,
    })
}

#[derive(Debug)]
pub struct Request {
    pub url: Uri,
    pub method: Method,
    pub headers: Option<HeaderMap>,
    pub body: Body,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            url: Default::default(),
            method: Method::GET,
            headers: None,
            body: Body::empty(),
        }
    }
}

pub struct Response {
    response: hyper::Response<hyper::Body>,
}

impl Response {
    pub async fn json<T: DeserializeOwned>(self) -> Result<T, Error> {
        let body = hyper::body::aggregate(self.response).await?;
        let json = serde_json::from_reader(body.reader())?;

        Ok(json)
    }

    pub async fn text(self) -> Result<String, Error> {
        let body = hyper::body::aggregate(self.response).await?;
        let text = String::from_utf8(body.chunk().to_vec())?;

        Ok(text)
    }
}
