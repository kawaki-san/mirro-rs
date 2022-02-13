use hyper::client::HttpConnector;
use openssl::ssl::{SslConnector, SslMethod};

#[cfg(feature = "archlinux")]
pub mod archlinux;
pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn http2_client() -> hyper::Client<hyper_openssl::HttpsConnector<HttpConnector>> {
    let builder = hyper::client::Client::builder();
    let mut ssl_build = SslConnector::builder(SslMethod::tls()).expect("creating ssl connector");
    ssl_build
        .set_alpn_protos(b"\x02h2")
        .expect("setting alpn protocols");
    let mut http = HttpConnector::new();
    http.enforce_http(false);
    let https = hyper_openssl::HttpsConnector::with_connector(http, ssl_build)
        .expect("creating https connector");
    builder.build::<_, hyper::Body>(https)
}
