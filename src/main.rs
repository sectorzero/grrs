use failure::Error;
use std::fs::File;
use std::io;
use std::io::Write;
use structopt::StructOpt;

use log::{info};

#[derive(StructOpt, Debug)]
struct SearchInput {
    /// the pattern to look for
    pattern: String,
    /// the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {

    env_logger::init();

    info!("Hello, GRRS!");

    // how is the error returned from here?
    let args = SearchInput::from_args();
    info!("{:?}", args);

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
    grrs::grep1(&args.pattern, r)?;

    // buffer example
    let mut buf: Vec<u8> = vec![];
    let sample: &[u8] = b"hello";
    buf.write_all(sample)?;

    // threads example
    grrs::do_some_thread_work();

    // thread example using rayon
    grrs::do_some_thread_work_using_rayon();

    Ok(())
}
