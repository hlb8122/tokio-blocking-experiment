pub async fn blocking(micro_seconds: u64) {
    let duration = std::time::Duration::from_micros(micro_seconds);
    std::thread::sleep(duration);
}

pub async fn non_blocking(micro_seconds: u64) {
    tokio::task::spawn_blocking(move || {
        let duration = std::time::Duration::from_micros(micro_seconds);
        std::thread::sleep(duration);
    })
    .await
    .unwrap();
}
