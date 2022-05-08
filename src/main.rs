use clap::Parser;
use rodio::OutputStream;
use std::fs::File;
use std::io::BufReader;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    /// The file you want to play
    #[clap()]
    file: String,

    /// Set the volume of the audio
    #[clap(short, long)]
    volume: f32,

    /// Set the speed of the audio
    #[clap(short, long)]
    speed: f32,
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

    let settings = AudioSettings::new(cli.volume, cli.speed);

    match play_audio(cli.file, settings) {
        Ok(_) => Ok(()),

        Err(_) => panic!("Error! Something went wrong, please try again."),
    }
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
