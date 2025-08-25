use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

pub struct AudioManager {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    music_sink: Option<Sink>,
}

impl AudioManager {
    pub fn new() -> Self {
        // Try to initialize audio output
        match OutputStream::try_default() {
            Ok((_stream, stream_handle)) => {
                println!("Audio system initialized successfully!");
                Self {
                    _stream,
                    stream_handle,
                    music_sink: None,
                }
            }
            Err(e) => {
                println!("Failed to initialize audio system: {}", e);
                // Create a dummy stream that won't work but won't crash
                let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|_| {
                    // This is a fallback that should never be reached, but prevents compilation errors
                    panic!("Cannot initialize audio at all")
                });
                Self {
                    _stream,
                    stream_handle,
                    music_sink: None,
                }
            }
        }
    }

    pub fn load_sounds(&mut self, _thread: &raylib::prelude::RaylibThread) {
        println!("Audio system ready");
    }

    pub fn start_background_music(&mut self) {
        self.stop_music(); // Stop any existing music first
        
        // Try to load and play the background music
        let music_paths = [
            "assets/music/Taylor.wav",
            "./assets/music/Taylor.wav",
            "Taylor.wav",
        ];

        for path in &music_paths {
            match self.load_and_play_music(path) {
                Ok(_) => {
                    println!("Started background music from: {}", path);
                    return;
                }
                Err(e) => {
                    println!("Failed to load music from {}: {}", path, e);
                    continue;
                }
            }
        }
        
        println!("Could not load background music from any path");
    }

    fn load_and_play_music(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Open the audio file
        let file = File::open(path)?;
        let buf_reader = BufReader::new(file);
        
        // Create a decoder for the audio file
        let source = Decoder::new(buf_reader)?;
        
        // Create a new sink for this music
        let sink = Sink::try_new(&self.stream_handle)?;
        
        // Make the music repeat indefinitely
        let repeating_source = source.repeat_infinite();
        
        // Set a reasonable volume
        sink.set_volume(0.3);
        
        // Append the source to the sink and start playing
        sink.append(repeating_source);
        
        // Store the sink so we can control it later
        self.music_sink = Some(sink);
        
        Ok(())
    }

    pub fn update(&mut self) {
        // Check if music is still playing, restart if needed
        if let Some(ref sink) = self.music_sink {
            if sink.empty() {
                println!("Music finished, restarting...");
                self.start_background_music();
            }
        }
    }

    pub fn set_music_volume(&mut self, volume: f32) {
        if let Some(ref sink) = self.music_sink {
            sink.set_volume(volume.clamp(0.0, 1.0));
            println!("Setting music volume to: {:.1}%", volume * 100.0);
        }
    }

    pub fn stop_music(&mut self) {
        if let Some(sink) = self.music_sink.take() {
            sink.stop();
            println!("Stopping background music");
        }
    }

    pub fn pause_music(&mut self) {
        if let Some(ref sink) = self.music_sink {
            sink.pause();
            println!("Pausing background music");
        }
    }

    pub fn resume_music(&mut self) {
        if let Some(ref sink) = self.music_sink {
            sink.play();
            println!("Resuming background music");
        }
    }

    pub fn is_music_playing(&self) -> bool {
        if let Some(ref sink) = self.music_sink {
            !sink.is_paused() && !sink.empty()
        } else {
            false
        }
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        self.stop_music();
        println!("Audio system shutting down");
    }
}
