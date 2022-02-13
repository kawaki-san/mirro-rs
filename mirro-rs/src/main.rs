#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mirrors_tx, _mirrors_rx) = tokio::sync::mpsc::channel(16);
    tokio::spawn(async move {
        let mirrors = match linux_mirrors::archlinux::mirrors().await {
            Ok(res) => res,
            Err(e) => {
                eprintln!("{e}");
                let local_file = include_str!("../../assets/arch_mirrors.json");
                serde_json::from_str(local_file).unwrap()
            }
        };
        mirrors_tx.send(mirrors).await
    });
    Ok(())
}
