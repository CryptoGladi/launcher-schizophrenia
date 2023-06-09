use discord_rpc_client::Client;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{System, SystemExt};

const CLIENT_ID: u64 = 1124038568470388877;

fn processes_found(name: &str) -> bool {
    let s = System::new_all();
    s.processes_by_exact_name(name).count() >= 1
}

pub fn run() -> anyhow::Result<Option<Client>> {
    if processes_found("Discord") || processes_found("Discord.exe") {
        log::info!("discord found");

        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
        let epoch = u64::try_from(since_the_epoch.as_millis())?;

        let mut drpc = Client::new(CLIENT_ID);

        drpc.start();
        drpc.set_activity(|act| {
            act.assets(|x| x.large_image("icon").large_text("Майнкрафт - моя жизнь!"))
                .instance(true)
                .details("Лучший сервер по майнкрафту!")
                .timestamps(|x| x.start(epoch))
        })?;

        log::info!("discord rpc client done!");
        return Ok(Some(drpc));
    } else {
        log::warn!("discord not found");
    }

    Ok(None)
}
