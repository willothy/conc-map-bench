use std::fmt::Debug;

use structopt::StructOpt;

mod adapters;
mod bench;
mod plot;
mod record;
mod workloads;

#[derive(Debug, StructOpt)]
enum Options {
    Bench(bench::Options),
    Plot(plot::Options),
}

fn main() {
    tracing_subscriber::fmt::init();

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();

    match Options::from_args() {
        Options::Bench(options) => rt.block_on(bench::bench(&options)),
        Options::Plot(options) => plot::plot(&options),
    }
}
