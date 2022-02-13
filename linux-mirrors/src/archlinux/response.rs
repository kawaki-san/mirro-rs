mod external {
    use serde::Deserialize;
    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ArchLinuxMirrors {
        pub cutoff: i64,
        #[serde(rename = "last_check")]
        pub last_check: String,
        #[serde(rename = "num_checks")]
        pub num_checks: i64,
        #[serde(rename = "check_frequency")]
        pub check_frequency: i64,
        pub urls: Vec<Url>,
        pub version: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Url {
        pub url: String,
        pub protocol: String,
        #[serde(rename = "last_sync")]
        pub last_sync: Option<String>,
        #[serde(rename = "completion_pct")]
        pub completion_pct: Option<f64>,
        pub delay: Option<i64>,
        #[serde(rename = "duration_avg")]
        pub duration_avg: Option<f64>,
        #[serde(rename = "duration_stddev")]
        pub duration_stddev: Option<f64>,
        pub score: Option<f64>,
        pub active: bool,
        pub country: String,
        #[serde(rename = "country_code")]
        pub country_code: String,
        pub isos: bool,
        pub ipv4: bool,
        pub ipv6: bool,
        pub details: String,
    }
}

pub mod internal {
    use std::collections::HashMap;

    use super::external;
    use serde::Deserialize;

    #[derive(Default, Debug, Clone, PartialEq, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[serde(from = "external::ArchLinuxMirrors")]
    pub struct ArchMirrors {
        pub cutoff: i64,
        #[serde(rename = "last_check")]
        pub last_check: String,
        #[serde(rename = "num_checks")]
        pub num_checks: i64,
        #[serde(rename = "check_frequency")]
        pub check_frequency: i64,
        pub countries: Vec<Url>,
        pub version: i64,
    }

    #[derive(Default, Debug, Clone, PartialEq)]
    pub struct Url {
        pub country: String,
        pub country_code: String,
        pub mirrors: Vec<Mirror>,
    }

    #[derive(Default, Debug, Clone, PartialEq)]
    pub struct Mirror {
        pub url: String,
        pub protocol: String,
        pub last_sync: Option<String>,
        pub completion_pct: Option<f64>,
        pub delay: Option<i64>,
        pub duration_avg: Option<f64>,
        pub duration_stddev: Option<f64>,
        pub score: Option<f64>,
        pub active: bool,
        pub isos: bool,
        pub ipv4: bool,
        pub ipv6: bool,
        pub details: String,
    }

    impl From<external::ArchLinuxMirrors> for ArchMirrors {
        fn from(root: external::ArchLinuxMirrors) -> Self {
            let mut mirrors = HashMap::<String, Vec<Mirror>>::new();
            for mirror in &root.urls {
                let external::Url {
                    url,
                    protocol,
                    last_sync,
                    completion_pct,
                    delay,
                    duration_avg,
                    duration_stddev,
                    score,
                    active,
                    country,
                    country_code: _,
                    isos,
                    ipv4,
                    ipv6,
                    details,
                } = mirror;
                mirrors
                    .entry(country.to_string())
                    .or_default()
                    .push(Mirror {
                        url: url.to_string(),
                        protocol: protocol.to_string(),
                        last_sync: last_sync.to_owned(),
                        completion_pct: *completion_pct,
                        delay: *delay,
                        duration_avg: *duration_avg,
                        duration_stddev: *duration_stddev,
                        score: *score,
                        active: *active,
                        isos: *isos,
                        ipv4: *ipv4,
                        ipv6: *ipv6,
                        details: details.to_string(),
                    })
            }
            let mirrors = mirrors
                .into_iter()
                .map(|(country, mirrors)| {
                    let mut url = Url {
                        country: country.clone(),
                        country_code: String::default(),
                        mirrors,
                    };

                    if let Some(loc) = root
                        .urls
                        .clone()
                        .into_iter()
                        .find(|x| x.country.eq(&country))
                    {
                        url.country_code = loc.country_code;
                    }
                    url
                })
                .collect();

            Self {
                cutoff: root.cutoff,
                last_check: root.last_check,
                num_checks: root.num_checks,
                check_frequency: root.check_frequency,
                countries: mirrors,
                version: root.version,
            }
        }
    }
}
