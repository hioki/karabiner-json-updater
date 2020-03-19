use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum BundleIdentifier {
    #[serde(rename = "com.googlecode.iterm2")]
    ITerm2,
    #[serde(rename = "com.microsoft.VSCode")]
    VSCode,
    #[serde(rename = "com.jetbrains.CLion")]
    CLion,
    #[serde(rename = "io.dynalist")]
    Dynalist,
    #[serde(rename = "com.github.atom")]
    Atom,
}
