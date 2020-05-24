use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Config {
    /// Search term
    pub query: String,

    /// Search case insensitive
    #[structopt(short, long)]
    pub ignore_case: bool,
}
