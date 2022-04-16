use std::sync::mpsc;
use std::thread;
use std::time::Instant;

const PI25DT : f64 = 3.141592653589793238462643_f64;
const INTERVALS : i64 = 10000000;

fn main() {
    let cpus = num_cpus::get() as i64;
    for cpu in 1..= cpus{
        for _ in 0..20 {
            let x = run(cpu);
            println!("{x}");
        }
    }
}

fn run(cpus: i64) -> String {
    let time1 = Instant::now();
    let (tx, rx) = mpsc::channel();

    let intervals = INTERVALS / cpus;
    let dx = 1.0 / INTERVALS as f64;

    for cpu in 1 ..= cpus  {
        let thread_tx = tx.clone();

        thread::spawn(move || {
            let mut inner_sum: f64 = 0.0;
            let mut x;

            let end = intervals * cpu;
            let start = end - intervals;


            for j in start ..= end {
                x = dx * (j as f64 - 0.5);
                inner_sum += 4.0 / (1.0 + x*x);
            }

            thread_tx.send(inner_sum).unwrap();
        });
    }

    let mut sum: f64 = 0.0;

    for _ in 1..= cpus {
        sum += rx.recv().unwrap()
    }

    let pi = dx * sum;
    let time2 = time1.elapsed();

    format!("{cpus}, {pi}, {:.24?}, {:.24?}", PI25DT-pi, time2.as_secs_f64())
}
