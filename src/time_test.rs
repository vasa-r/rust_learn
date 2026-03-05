use ::std::thread;
use ::std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

pub fn test_time() {
    let wait_duaration = Duration::from_secs(10);
    thread::sleep(wait_duaration);
    let start = Instant::now();
    let mut sum: u64 = 0;

    for i in 1..1_000_000 {
        sum = sum + i;
    }

    let duration = start.elapsed();
    println!("{:?}", duration);

    let system_time = SystemTime::now();
    let now = system_time.duration_since(UNIX_EPOCH).unwrap();
    println!("{:?} {:?}", system_time, now.as_secs());
}
