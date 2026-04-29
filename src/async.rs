use std::sync::{Mutex, OnceLock};
use std::collections::BinaryHeap;
use std::thread;
use std::time::Duration;
use crate::RustEngine;

#[derive(Clone)]
pub struct Job {
    pub priority: i32,
    pub func: String,
    pub data: String,
}


use std::cmp::Ordering;

impl Ord for Job {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Job {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Job {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for Job {}

static QUEUE: OnceLock<Mutex<BinaryHeap<Job>>> = OnceLock::new();

fn queue() -> &'static Mutex<BinaryHeap<Job>> {
    QUEUE.get_or_init(|| Mutex::new(BinaryHeap::new()))
}



fn execute(job: Job) {
    match job.func.as_str() {
    	"println" => RustEngine::println(&job.data),
        _ => println!("❌ Unknown function: {}", job.func),
    }
}


pub fn start_workers(size: usize) {
    for _id in 0..size {
        thread::spawn(move || loop {
            let job = {
                let mut q = queue().lock().unwrap();
                q.pop()
            };

            if let Some(job) = job {
               // println!("⚙ Worker {} -> priority {}", id, job.priority);
                execute(job);
            }

            thread::sleep(Duration::from_millis(0));
        });
    }
}


pub fn push_job(func: String, data: String, priority: i32) {
    let mut q = queue().lock().unwrap();

    q.push(Job {
        func,
        data,
        priority,
    });
}

