use std::{fs::OpenOptions, io::Write};

use super::state::SelectedCountry;

pub(crate) async fn export_mirrors(
    selected_countries: Vec<SelectedCountry>,
    rate: bool,
) -> super::AppReturn {
    match rate {
        true => todo!(),
        false => tokio::spawn(async move {
            let mut fs = OpenOptions::new()
                .create(true)
                .append(true)
                .open("my_mirrors")
                .unwrap();
            for i in selected_countries.iter() {
                for x in i.country.mirrors.iter() {
                    writeln!(fs, "{}", &x.url).unwrap();
                }
            }
        }),
    };
    super::AppReturn::Continue
}
