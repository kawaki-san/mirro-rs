use serde::Deserialize;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MirrorsConfig {
    pub colours: Option<Colours>,
    pub icons: Option<Icons>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Colours {
    #[serde(rename = "action_key")]
    pub action_key: Option<String>,
    #[serde(rename = "available-mirrors")]
    pub available_mirrors: Option<AvailableMirrors>,
    pub countries: Option<Countries>,
    pub mirrors: Option<Mirrors>,
    pub info: Option<Info>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AvailableMirrors {
    pub heading: Option<String>,
    pub border: Option<String>,
    pub highlight: Option<String>,
    pub reverse: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Countries {
    pub heading: Option<String>,
    pub border: Option<String>,
    pub highlight: Option<String>,
    pub reverse: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mirrors {
    pub heading: Option<String>,
    pub border: Option<String>,
    pub reverse: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub os: Option<String>,
    pub app: Option<String>,
    pub countries: Option<String>,
    pub mirrors: Option<String>,
    #[serde(rename = "last_checked")]
    pub last_checked: Option<String>,
    pub now: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Icons {
    pub os: Option<String>,
    pub countries: Option<String>,
    pub mirrors: Option<String>,
    #[serde(rename = "last_checked")]
    pub last_checked: Option<String>,
    pub now: Option<String>,
}
