pub struct App {
    pub track_map_created: bool,
    pub desktop_detected: bool,
    pub playlist_detected: bool,
    pub drive_detected: bool,
    
    pub drive_letter: Option<String>,
    pub current_file: Option<String>,
    
    pub status_message: String,
    pub error_message: Option<String>,
    
    pub progress: f64, // 0.0 -> 1.0
    pub is_mp3_copying: bool,

    pub files_cleared: usize,
    pub files_total: usize
}

impl App {
    pub fn new() -> Self {
        Self {
            is_mp3_copying: false,

            track_map_created: false,
            desktop_detected: false,
            playlist_detected: false,

            drive_detected: false,
            drive_letter: None,
            status_message: "Starting...".into(),
            error_message: None,
            progress: 0.0,
            current_file: None,
            
            files_cleared: 0,
            files_total: 0,
        }
    }

    // Setters
    pub fn SetError(&mut self, msg: impl Into<String>) {
        self.error_message = Some(msg.into());
    }

    pub fn SetDriveLetter(&mut self, msg: impl Into<String>) {
        self.drive_letter = Some(msg.into());
    }

    pub fn SetStatusMessage(&mut self, msg: impl Into<String>) {
        self.status_message = msg.into();
    }

    pub fn UpdateProgress(&mut self, value: impl Into<f64>) {
        self.progress = value.into();
    }

    pub fn SetCurrentFile(&mut self, msg: impl Into<String>) {
        self.current_file = Some(msg.into());
    }

    pub fn SetDriveStatus(&mut self, status: impl Into<bool>) {
        self.drive_detected = status.into();
    }

    pub fn SetDesktopStatus(&mut self, status: impl Into<bool>) {
        self.desktop_detected = status.into();
    }

    pub fn SetTrackMapStatus(&mut self, status: impl Into<bool>) {
        self.track_map_created = status.into();
    }

    pub fn SetPlaylistStatus(&mut self, status: impl Into<bool>) {
        self.playlist_detected = status.into();
    }
}
