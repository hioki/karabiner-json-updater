use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum BundleIdentifier {
    #[serde(rename = "com.googlecode.iterm2")]
    ITerm2,
    #[serde(rename = "com.microsoft.VSCode")]
    VSCode,
    #[serde(rename = "com.jetbrains.CLion")]
    CLion,
    #[serde(rename = "io.dynalist")]
    Dynalist, // https://help.dynalist.io/article/91-keyboard-shortcut-reference
    #[serde(rename = "com.tinyspeck.slackmacgap")]
    Slack,
    #[serde(rename = "com.google.Chrome")]
    GoogleChrome,
    #[serde(rename = "notion.id")]
    Notion,
    #[serde(rename = "com.clickup.desktop-app")]
    ClickUp,
    #[serde(rename = "com.jetbrains.cwm.guest")]
    JetBrainsClient,
    #[serde(rename = "com.openai.chat")]
    ChatGPT,
}
