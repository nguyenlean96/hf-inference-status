use reactive_stores::Store;

#[derive(Debug, Clone, Store)]
pub struct AppState {
    pub options_panel_enabled: bool,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            options_panel_enabled: false,
        }
    }
}
