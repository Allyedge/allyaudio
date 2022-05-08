use clap::Parser;
use rodio::OutputStream;
use std::fs::File;
use std::io::{BufReader, Cursor};

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

struct AudioSettings {
    volume: f32,
    speed: f32,
}

impl AudioSettings {
    fn new(volume: f32, speed: f32) -> AudioSettings {
        AudioSettings { volume, speed }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = CLI::parse();
    let settings = AudioSettings::new(cli.volume.unwrap_or(1.0), cli.speed.unwrap_or(1.0));

    if cli.file_or_url.starts_with("http") {
        match play_audio_from_url(cli.file_or_url, settings) {
            Ok(_) => println!("Playing..."),
            Err(_) => panic!("Error! Something went wrong, please try again."),
        }
    } else {
        match play_audio(cli.file_or_url, settings) {
            Ok(_) => println!("Playing..."),
            Err(_) => panic!("Error! Something went wrong, please try again."),
        }
    }

    Ok(())
}

fn play_audio_from_url(
    url: String,
    settings: AudioSettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(&url).unwrap();

    let cursor = Cursor::new(response.bytes().unwrap());

    let (_stream, stream_handle) = OutputStream::try_default()?;

    let sink = stream_handle.play_once(cursor)?;

    sink.set_volume(settings.volume);
    sink.set_speed(settings.speed);
    sink.sleep_until_end();

    Ok(())
}

fn play_audio(file: String, settings: AudioSettings) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let file = BufReader::new(File::open(file)?);

    let sink = stream_handle.play_once(file)?;

    sink.set_volume(settings.volume);
    sink.set_speed(settings.speed);
    sink.sleep_until_end();

    Ok(())
}
