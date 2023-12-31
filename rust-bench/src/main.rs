use std::{
    sync::{atomic::{AtomicI64, Ordering}, Arc, Mutex},
    time,
};

#[tokio::main]
async fn main() {
    let count = Arc::new(AtomicI64::new(0));
    let finish = Arc::new(Mutex::new(THREAD_COUNT));
    let running_task: Vec<_> = (0..THREAD_COUNT).map(|_|{
        let count = count.clone();
        let finish = finish.clone();
        tokio::spawn(sub_tasks(count, finish))
    }).collect();

    let count_task = tokio::spawn(print_result(count.clone(), finish));

    for t in running_task {
        t.await.unwrap();
    }

    count_task.await.unwrap();

    let task_finished = count.load(Ordering::Relaxed);
    let task_required = TASK_COUNT * THREAD_COUNT;
    assert_eq!(task_required , task_finished);
    println!("Finish finished {task_finished} tasks, require {task_required}");
}

const TASK_SPAN: i64 = 15;
const TASK_COUNT: i64 = 15 * 1000 / TASK_SPAN;
const THREAD_COUNT: i64 = 10_000;

async fn sub_tasks(count: Arc<AtomicI64>, finish: Arc<Mutex<i64>>) {
    let task_span = time::Duration::from_millis(TASK_SPAN as u64);
    for _ in 0..TASK_COUNT {
        tokio::time::sleep(task_span).await;
        count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    *finish.lock().unwrap() -= 1;
}

async fn print_result(count: Arc<AtomicI64>, finish: Arc<Mutex<i64>>) {
    let mut last_count = 0;
    let start_time = time::Instant::now();
    loop {
        tokio::time::sleep(time::Duration::from_secs(1)).await;
        let current_count = count.load(std::sync::atomic::Ordering::Relaxed);
        let span = time::Instant::now().duration_since(start_time);
        let change = current_count - last_count;
        println!("{change} tasks per second at {span:?}");
        last_count = current_count;
        if *finish.lock().unwrap() == 0 {
            break;
        }
    }
}
