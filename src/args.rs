use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Downloads European dust concentration and dust load forecasts from the University of Athens website and combines them into GIF files.",
    arg_required_else_help = true
)]
pub struct Args {
    #[arg(
        long,
        help = "Download Dust Load data into the given file.",
        default_missing_value = "dust_load.gif",
        num_args = 0..=1,
        value_name = "OUTPUT FILE"
    )]
    pub dust_load: Option<String>,

    #[arg(
        long,
        help = "Download Dust Concentration data into the given file.",
        default_missing_value = "dust_concentration.gif",
        num_args = 0..=1,
        value_name = "OUTPUT FILE"
    )]
    pub dust_concentration: Option<String>,

    #[arg(
        short,
        long,
        help = "Frames per second of the resulting GIF.",
        default_value = "5"
    )]
    pub fps: u8,

    #[arg(
        short,
        long,
        help = "Save the intermediate PNG frame files with their original filenames."
    )]
    pub save_intermediate: bool,
}
