// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the threads spawned on line 20 are completing jobs while the main thread is
// monitoring progress until 10 jobs are completed.

// I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    for i in 1..=10 {
        let status_ref = Arc::clone(&status);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(250 * i));
            status_ref.jobs_completed += 1;
        });
    }
    while status.jobs_completed < 10 {
        println!("waiting for {} jobs ({} jobs running)... ", 
            (10 - status.jobs_completed), 
            (Arc::strong_count(&status) - 1) // subtract one for refrence in _this_ thread
        );
        thread::sleep(Duration::from_millis(500));
    }
}
