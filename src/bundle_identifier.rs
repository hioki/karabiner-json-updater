use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BundleIdentifier {
    #[serde(rename = "com.googlecode.iterm2")]
    ITerm2,
    #[serde(rename = "com.microsoft.VSCode")]
    VSCode,
}
