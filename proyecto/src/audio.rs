use raylib::prelude::*;

pub struct AudioManager {
    audio_initialized: bool,
}

impl AudioManager {
    pub fn new(rl: &mut RaylibHandle) -> Self {
        
        let mut audio_initialized = false;
        
        
        unsafe {
            raylib::ffi::InitAudioDevice();
            if raylib::ffi::IsAudioDeviceReady() {
                audio_initialized = true;
                println!("Audio device initialized successfully!");
            } else {
                println!("Failed to initialize audio device");
            }
        }

        if audio_initialized {
            println!("Audio system ready - sound effects will use system beeps");
        } else {
            println!("Warning: Could not initialize audio device");
        }

        Self {
            audio_initialized,
        }
    }

    pub fn start_background_music(&mut self) {
        if self.audio_initialized {
            println!("Background music would start here (using system sounds as fallback)");
        }
    }

    pub fn update(&mut self) {
        
    }

    pub fn play_coin_sound(&self) {
        if self.audio_initialized {
            
            self.play_system_beep();
            println!("Coin collected! *ding*");
        } else {
            println!("Coin collected! (audio not available)");
        }
    }

    fn play_system_beep(&self) {
        #[cfg(target_os = "linux")]
        {
            use std::process::Command;
            if Command::new("paplay").arg("/usr/share/sounds/alsa/Front_Left.wav").output().is_err() {
                let _ = Command::new("sh").args(&["-c", "echo -e '\\a'"]).output();
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            use std::process::Command;
            let _ = Command::new("powershell")
                .args(&["-c", "[console]::beep(800,150)"])
                .output();
        }
        
        #[cfg(target_os = "macos")]
        {
            use std::process::Command;
            let _ = Command::new("afplay")
                .args(&["/System/Library/Sounds/Glass.aiff"])
                .output();
        }
    }

    pub fn set_music_volume(&self, volume: f32) {
        if self.audio_initialized {
            println!("Setting music volume to: {:.1}%", volume * 100.0);
        }
    }

    pub fn stop_music(&mut self) {
        if self.audio_initialized {
            println!("Stopping background music");
        }
    }
}

impl Drop for AudioManager {
    fn drop(&mut self) {
        if self.audio_initialized {
            unsafe {
                raylib::ffi::CloseAudioDevice();
            }
        }
    }
}
