use systemd::daemon::{self, State};

impl Status {
    pub fn available_spi_module() {
        daemon::notify(true, &[State::Status("module lora disponible".to_string())]);
    }
}
