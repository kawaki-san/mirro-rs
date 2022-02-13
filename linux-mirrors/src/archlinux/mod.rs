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
