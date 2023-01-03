use clap::Parser;
use pc_bench::Algorithm;
use tracing_subscriber::fmt::format::FmtSpan;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Algorithm to test
    #[arg(value_enum)]
    algo: Algorithm,

    /// Log₂ of the maximum number of values to test
    #[arg(default_value_t = 20)]
    max_exponent: usize,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .compact()
        .init();

    let cli = Args::parse();
    
    pc_bench::run(cli.algo, cli.max_exponent);
}
