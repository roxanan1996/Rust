use threadpool::ThreadPool;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let n_workers: usize = args[1].parse().unwrap();
    println!("{:?}", n_workers);

    let pool = ThreadPool::new(n_workers);
    pool.execute(|| println!("hello"));
    pool.execute(|| println!("world"));
    pool.execute(|| println!("foo"));
    pool.execute(|| println!("bar"));

    pool.join();

}
