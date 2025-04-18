// export RUST_LOG=trace

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init_timed(); // todo: date and message formatting
    trace!("a trace example");
    debug!("deboogging");
    info!("such information");
    warn!("o_O");
    error!("boom");
}
