#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mirrors = match linux_mirrors::archlinux::mirrors().await {
        Ok(mirrors) => mirrors,
        Err(e) => {
            eprintln!("{e}");
            let bytes = include_bytes!("../../assets/arch_mirrors.json");
            let str = String::from_utf8_lossy(bytes);
            serde_json::from_str(&str)?
        }
    };
    println!("Hello, world! {}", mirrors.countries.len());
    Ok(())
}
