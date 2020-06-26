use clap::arg_enum;
use structopt::StructOpt;

arg_enum!{
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum ContentCategory {
        all,
        sports,
        shows,
        news,
    }
}

#[derive(StructOpt)]
pub struct Config {
    /// Search term
    pub query: String,

    /// Search case insensitive
    #[structopt(short, long)]
    pub ignore_case: bool,


    #[structopt(long, env = "PLEX_TOKEN", hide_env_values = true)]
    pub plex_token: String,

    #[structopt(long, env = "PLEX_HOSTNAME", hide_env_values = true)]
    pub plex_hostname: String,


    #[structopt(long, env = "PLEX_PORT", default_value = "32400")]
    pub plex_port: String,

    #[structopt(long, env = "PLEX_GUIDE_DATA_CACHE", default_value = "/var/tmp/plex_guide_data_cache")]
    pub plex_guide_data_cache: String,

    #[structopt(long, env = "PLEX_ENABLE_GUIDE_DATA_CACHE")]
    pub plex_enable_guide_data_cache: bool,

    #[structopt(long, possible_values = &ContentCategory::variants(), case_insensitive = true, default_value = "all")]
    pub category: ContentCategory,
}
