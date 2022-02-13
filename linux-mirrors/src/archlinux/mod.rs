use self::internal::ArchMirrors;
use crate::http2_client;

use super::Result;
mod response;
use hyper::body::Buf;
pub use response::internal;

const ARCHLINUX_URL: &str = "https://archlinux.org/mirrors/status/json/";
#[cfg(test)]
mod tests {
    use crate::{archlinux::ARCHLINUX_URL, http2_client};

    #[tokio::test]
    async fn archlinux() -> Result<(), Box<dyn std::error::Error>> {
        let client = http2_client();
        let response = client.get(ARCHLINUX_URL.parse()?).await?;
        assert_eq!(response.status(), hyper::StatusCode::OK);
        Ok(())
    }
}

pub async fn mirrors() -> Result<ArchMirrors> {
    let uri = ARCHLINUX_URL.parse()?;
    let res = http2_client().get(uri).await?;
    let body = hyper::body::aggregate(res).await?;
    let response: ArchMirrors = serde_json::from_reader(body.reader())?;
    Ok(response)
}
