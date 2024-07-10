use std::time::Instant;
use tracing::{debug, info};

// тесты писала нейросеть. все вопросы к ней.

pub(crate) async fn test_speed(mut iterations: i32) {
    if iterations <= 1000 { iterations = 100_000_000 }
    let iters: i32 = iterations;

    let storage = super::db::Storage::new();

    let start_insert = Instant::now();
    for i in 0..=iterations {
        storage.set(i, format!("value{}{}{}", i, i, i));
    }
    let duration_insert = start_insert.elapsed();

    let start_get = Instant::now();
    for i in 0..=iterations {
        let _ = storage.get(&i);
    }
    let duration_get = start_get.elapsed();

    let start_remove = Instant::now();
    for i in 0..=iterations {
        if storage.get(&i).is_some() {
            let _ = storage.remove(i);
        }
    }
    let duration_remove = start_remove.elapsed();

    debug!("Storage size: {:?}", iters);
    info!("Insert Time: {:?}", duration_insert);
    info!("Get Time: {:?}", duration_get);
    info!("Remove time {:?}", duration_remove);
}