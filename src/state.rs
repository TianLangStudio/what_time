use ratatui::style::Color;

#[derive(Debug, Default)]
pub struct AppState {
  is_show_help: bool,
  time_fg_color: Color,
  time_bg_color: Color,
}

impl AppState {
  pub fn new() -> Self {
    Self {
      is_show_help: true,
      time_fg_color: Color::Yellow,
      .. AppState::default()
    }
  }
  pub fn get_is_show_help(&self) -> bool {
    self.is_show_help
  }

  pub fn get_time_fg_color(&self) -> &Color {
    &self.time_fg_color
  }

  pub fn get_time_bg_color(&self) -> &Color {
    &self.time_bg_color
  }

  pub fn toggle_is_show_help(&mut self) {
    self.is_show_help = !self.is_show_help;
  }
  pub fn set_time_fg_color(&mut self, color: &Color) {
    self.time_fg_color = *color;
  }
}
