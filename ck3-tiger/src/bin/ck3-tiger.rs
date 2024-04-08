use anyhow::Result;
use tiger_bin_shared::{run, GameConsts, PackageEnv};

fn main() -> Result<()> {
    run(
        GameConsts {
            name: "Crusader Kings 3",
            name_short: "CK3",
            version: "1.12.3 (SCYTHE)",
            dir: "steamapps/common/Crusader Kings III",
            app_id: "1158310",
            signature_file: "game/events/witch_events.txt",
        },
        PackageEnv { name: env!("CARGO_PKG_NAME"), version: env!("CARGO_PKG_VERSION") },
    )
}
