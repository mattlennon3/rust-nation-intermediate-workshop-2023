use std::sync::Arc;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

type ThreadPoolHandle = JoinHandle<Box<dyn FnOnce()>>;

struct ThreadPool {
    handles: Vec<ThreadPoolHandle>,
    handler: Arc<>,
}

impl ThreadPool {
    // handles: Vec<N>;

    fn new(number_of_threads: usize) -> Self {
        let (sender, receiver) = channel::<ThreadPoolHandle>();
        let receiver = Arc::new(receiver);
        for n in number_of_threads {
            let waiting_thread = thread::spawn();
        }
        ThreadPool {
            handles: Vec::with_capacity(number_of_threads),
        }
    }

    fn execute<T>(&self, task: T)
    where
        T: FnOnce() + Send + 'static
     {
        let box_task = Box::new(task);
        let boxed_handle = thread::spawn(box_task);
        // iterate through threads
        // let boxed_handle = thread::spawn(task);
        // let handle = ;
        // sender
        self.handles.push(boxed_handle);
    }
}

fn main() {
    let pool = ThreadPool::new(10);

    pool.execute(|| {
        thread::sleep(Duration::from_secs(2));
        println!("SLOW Hello from thread");
    });
    for i in 0..15 {
        pool.execute(move || {
            println!("FAST Hello from thread for task: {}", i);
        });
    }

    // First we're making sure enough time is given to threads to execute the tasks
    // Then, replace this line with the `stop` method.
    thread::sleep(Duration::from_secs(3));
    // pool.stop();
}
