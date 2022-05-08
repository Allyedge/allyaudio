use rodio::OutputStream;
use std::fs::File;
use std::io::{BufReader, Cursor};

pub struct AudioSettings {
    volume: f32,
    speed: f32,
}

impl AudioSettings {
    pub fn new(volume: f32, speed: f32) -> AudioSettings {
        AudioSettings { volume, speed }
    }
}

pub fn play_audio_from_url(
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

pub fn play_audio(file: String, settings: AudioSettings) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;

    let file = BufReader::new(File::open(file)?);

    let sink = stream_handle.play_once(file)?;

    sink.set_volume(settings.volume);
    sink.set_speed(settings.speed);
    sink.sleep_until_end();

    Ok(())
}
