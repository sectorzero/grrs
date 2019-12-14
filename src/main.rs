use failure::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use structopt::StructOpt;

use indicatif::ProgressBar;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;

#[derive(StructOpt, Debug)]
struct SearchInput {
    /// the pattern to look for
    pattern: String,
    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    println!("Hello, GRRS!");

    // how is the error returned from here?
    let args = SearchInput::from_args();
    println!("{:?}", args);

    /* unbuffered impl
    // reading from file to string
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read from file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    */

    let f = File::open(&args.path).unwrap();
    let r = io::BufReader::new(f);
    grep1(&args.pattern, r)?;

    let mut buf: Vec<u8> = vec![];
    let sample: &[u8] = b"hello";
    buf.write_all(sample)?;

    // threads example
    do_some_thread_work();

    Ok(())
}

fn grep1<R>(target: &str, reader: R) -> Result<(), Error>
where
    R: BufRead,
{
    for line_result in reader.lines() {
        let line: String = line_result?;
        if line.contains(target) {
            println!("{}", line);
        }
    }
    Ok(())
}

fn do_some_thread_work() {

    // random number generator
    let rng = thread_rng();

    // atomic counter
    let work_completed_count = Arc::new(AtomicUsize::new(0));

    // some variable work simulation
    let mut work_assigned_count: usize = 0;

    // spawn work
    let mut t_handles: Vec<JoinHandle<()>> = vec![];
    {
        let mut work_counts = Uniform::new_inclusive(7, 11).sample_iter(rng);

        for _ in 0..10 {
            let t_counter = work_completed_count.clone();

            let t_work_count: usize = work_counts.next().unwrap();
            work_assigned_count += t_work_count;

            t_handles.push(spawn(move || {

                let t_rng = thread_rng();
                let mut t_wait_times = Uniform::new_inclusive(100, 1000).sample_iter(t_rng);

                for _ in 0..t_work_count {
                    let t_wait: u64 = t_wait_times.next().unwrap();
                    sleep(Duration::from_millis(t_wait));
                    t_counter.fetch_add(1, Ordering::SeqCst);
                }

            }));
        }
    }

    // progress bar
    let pb = ProgressBar::new(work_assigned_count as u64);
    loop {
        sleep(Duration::from_millis(100));

        let p = work_completed_count.load(Ordering::SeqCst); pb.set_position(p as u64);
       
        if p >= work_assigned_count {
            break;
        }
    }

    // wait and verify completed work count
    for h in t_handles {
        h.join().unwrap();
    }

    assert_eq!(
        work_completed_count.load(Ordering::SeqCst), 
        work_assigned_count);
}
