mod letter;
mod state;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph},
};
use std::time::Duration;
use ratatui::layout::Flex;
use ratatui::prelude::*;
use ratatui::style::Styled;
use crate::letter::Letter;
use crate::state::AppState;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app_state = AppState::new();
    loop {
        terminal.draw(|frame: &mut Frame| {
            render(frame, &app_state)
        })?;
        let has_event = event::poll(Duration::from_millis(100))?;
        if has_event {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    match key_event.code {
                        KeyCode::Char('q') => { break }
                        KeyCode::Char('h') => {app_state.toggle_is_show_help()}
                        KeyCode::Char('y') => {app_state.set_time_fg_color(&Color::Yellow)}
                        KeyCode::Char('b') => {app_state.set_time_fg_color(&Color::Blue)}
                        KeyCode::Char('r') => {app_state.set_time_fg_color(&Color::Red)}
                        KeyCode::Char('w') => {app_state.set_time_fg_color(&Color::White)}
                        _ => {continue}
                    }

                }
                _ => {}
            };
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    //layout
    let frame_area = frame.area();
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(10),
            Constraint::Min(0),
        ])
        .split(frame_area);
    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
        ])
        .flex(Flex::Center)
        .split(outer_layout[1]);
    //render time letters
    let local_time = chrono::Local::now();
    let local_time = local_time.format("%Y/%m/%d %H:%M:%S").to_string();
    let time_fg_color = app_state.get_time_fg_color();
    let time_bg_color = app_state.get_time_bg_color();
    let time_texts = &local_time[11..].chars().map(|c|{
        let s: &str = &c.to_string();
        let letter = Letter::from(s);
        Text::raw(letter.as_str()).style(Style::default().bg(*time_bg_color).fg(*time_fg_color))
    }).collect::<Vec<Text>>();
    for (i, text) in time_texts.iter().enumerate() {
        frame.render_widget(text, inner_layout[i]);
    }
    let local_time_text = Text::from(vec![Line::from(vec![local_time.set_style(
        Style::new().fg(*time_fg_color))
    ])]);
    let mut p = Paragraph::new(local_time_text).centered();
    //set help information
    if app_state.get_is_show_help() {
        let title = Line::from(" What is the time? ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".white().bold(),
                                           " Hide or Show Help".into(), "<H>".white().bold(),
                                           " Set Text Color:".into(),
                                           " Yellow".into(), "<Y> ".yellow().bold(),
                                           " Blue".into(), "<B>".blue().bold(),
                                           " Red".into(), "<R>".red().bold(),
                                           " White".into(), "<W>".white().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);
        p = p.block(block);
    }
    frame.render_widget(p, frame.area());
}
