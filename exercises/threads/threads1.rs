// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :) 
// Why 6 lines, you ask? because the program will spawn one new thread that will increment
// the Jobstatus on 250ms intervals. At the same time, our original thread will check
// the Jobstatus at 500ms intervals. So, this count should be ~0 on the first iteration of
// the `while` loop; ~2 on the second iteration; ~4 on the third iteration; finally, 
// ~10 on the sixth. Why? Because by the time our main thread peeks at the JobStatus counter,
// our second spawned thread will have already run two incremental operations on it.

// I AM NOT DONE

use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // TODO: Change the line below
    let status = Arc::new(JobStatus { jobs_completed: 0 });
    let status_shared = status.clone();
    // The code below spawns a single new thread that will run a for-loop code block 10 times.
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            // TODO: change the line below
            status_shared.jobs_completed += 1;
        }
    });
    // The code below will check the count on JobStatus on 500ms intervals.
    // TODO: Change the line below
    while status.jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
