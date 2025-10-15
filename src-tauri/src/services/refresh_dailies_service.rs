use chrono::Timelike;
use std::time::Duration;
use tokio::time::sleep;
use tauri::{AppHandle, Emitter};
use serde_json::json;

pub async fn init(app: AppHandle) {
    tokio::spawn(async move {
        loop {
            app.emit("refresh_dailies", json!(null)).unwrap();

            let now = chrono::Local::now();
            let next_day = (now + chrono::Duration::days(1))
                .with_hour(0)
                .unwrap()
                .with_minute(0)
                .unwrap()
                .with_second(0)
                .unwrap()
                .with_nanosecond(0)
                .unwrap();

            let dur: Duration = (next_day - now).to_std().unwrap();
            println!("{:?}", dur);
            sleep(dur).await;
        }
    });
}