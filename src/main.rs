use clap::Parser;
use std::panic;

mod play;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    /// The file or the URL you want to play
    #[clap()]
    file_or_url: String,

    /// Set the volume of the audio
    #[clap(short, long)]
    volume: Option<f32>,

    /// Set the speed of the audio
    #[clap(short, long)]
    speed: Option<f32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    panic::set_hook(Box::new(|_| {
        println!("Error! Something went wrong, please try again.");
    }));

    let cli = CLI::parse();

    let settings =
        play::models::AudioSettings::new(cli.volume.unwrap_or(1.0), cli.speed.unwrap_or(1.0));

    if cli.file_or_url.starts_with("http") {
        match play::play_audio_from_url(cli.file_or_url, settings) {
            Ok(_) => Ok(()),
            Err(_) => panic!(),
        }
    } else {
        match play::play_audio(cli.file_or_url, settings) {
            Ok(_) => Ok(()),
            Err(_) => panic!(),
        }
    }
}
