mod letter;

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
use crate::letter::Letter;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        let has_event = event::poll(Duration::from_millis(100))?;
        if has_event {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    if let KeyCode::Char('q') = key_event.code { break }
                }
                _ => {}
            };
        }
    }
    Ok(())
}

fn render(frame: &mut Frame) {
    let title = Line::from(" What is the time? ".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
    let local_time = chrono::Local::now();
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
    let local_time = local_time.format("%Y/%m/%d %H:%M:%S").to_string();
    let time_texts = &local_time[11..].chars().map(|c|{
        let s: &str = &c.to_string();
        let letter = Letter::from(s);
        Text::raw(letter.as_str()).style(Style::default().bg(Color::Black).fg(Color::Yellow))
    }).collect::<Vec<Text>>();
    for (i, text) in time_texts.iter().enumerate() {
        frame.render_widget(text, inner_layout[i]);
    }
    let local_time_text = Text::from(vec![Line::from(vec![local_time.yellow()])]);

    let p = Paragraph::new(local_time_text).centered().block(block);

    frame.render_widget(p, frame.area());
}
