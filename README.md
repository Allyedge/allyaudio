# Allyaudio

[![Crates.io Version](https://img.shields.io/crates/v/allyaudio.svg)](https://crates.io/crates/allyaudio)
[![Crates.io Downloads](https://img.shields.io/crates/d/allyaudio.svg)](https://crates.io/crates/allyaudio)

A simple audio player CLI tool written in Rust.

## Installation

```sh
> cargo install allyaudio
```

## Usage

```sh
# Play a local audio file
> allyaudio example.mp3

# Play audio from an URL
> allyaudio https://example.com/audio.mp3

# Set the volume and the speed of the audio
> allyaudio --volume=2 --speed=0.5 example.mp3

# Help
> allyaudio --help

# Version
> allyaudio --version
```
