pub mod main_mod {
    use std::thread;
    use std::time::Duration;
    use chrono::{Timelike, Utc};
    use serenity::client::Context;
    use crate::loops::pfp_mod::{update_pfp, reset_pfp_cycle};

    pub fn main_loop(context: Context) {
        loop {
            let now = Utc::now();
            let minute = now.minute();

            if minute == 00 {
                update_pfp(context.to_owned());
            }
            if minute == 01 {
                reset_pfp_cycle();
            }
            thread::sleep(Duration::from_secs(60));
        }
    }
}