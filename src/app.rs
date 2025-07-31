pub struct App {
    pub drive_detected: bool,
    pub drive_letter: Option<String>,
    pub status_message: String,
    pub error_message: Option<String>,
    pub progress: f64, // 0.0 -> 1.0
    pub current_file: Option<String>
}

impl App {
    pub fn new() -> Self {
        Self {
            drive_detected: false,
            drive_letter: None,
            status_message: "Starting...".into(),
            error_message: None,
            progress: 0.0,
            current_file: None
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
}

enum AppState {
    Idle,
    Scanning,
    Copying,
    Finished,
    Error,
}
