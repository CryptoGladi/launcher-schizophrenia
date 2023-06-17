use crate::exit_unwrap::ExitUnwrap;
use open::that as open_url;

const DISCORD_URL: &str = "https://discord.gg/JSM8f42Tvx";

#[tauri::command]
pub fn open_url_discord() {
    open_url(DISCORD_URL).exit_unwrap();
}
