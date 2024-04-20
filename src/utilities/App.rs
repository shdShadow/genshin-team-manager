pub mod app {

    use std::{default, io};

    use crate::utilities::tui;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::{
        buffer::Buffer,
        layout::{Alignment, Rect},
        style::Stylize,
        symbols::border,
        text::{Line, Text},
        widgets::{
            block::{Position, Title},
            Block, Borders, Paragraph, Widget,
        },
        Frame,
    };
    #[derive(Debug, Default)]
    pub enum AppState {
        #[default]
        MainWindow,
    }

    #[derive(Debug, Default)]
    pub struct App {
        exit: bool,
        state: AppState,
    }

    impl App {
        pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
            while !self.exit {
                terminal.draw(|frame| self.render_frame(frame))?;
                self.handle_events()?;
            }
            Ok(())
        }

        fn render_frame(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.size());
        }

        fn handle_events(&mut self) -> io::Result<()> {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
            Ok(())
        }

        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                _ => {}
            }
        }

        fn exit(&mut self) {
            self.exit = true;
        }
    }

    impl Widget for &App {
        fn render(self, area: Rect, buf: &mut Buffer) {
            match self.state {
                AppState::MainWindow => {
                    let title = Title::from(" Counter app Tutorial ".bold());
                    let instructions = Title::from(Line::from(vec![
                        " Decrement ".into(),
                        "<Left>".blue().bold(),
                        " Increment ".into(),
                        "<Right>".blue().bold(),
                        " Quit ".into(),
                        " <Q> ".blue().bold(),
                    ]));
                    let block = Block::default()
                        .title(title.alignment(Alignment::Center))
                        .title(
                            instructions
                                .alignment(Alignment::Center)
                                .position(Position::Bottom),
                        )
                        .borders(Borders::ALL)
                        .border_set(border::THICK);
                    let counter_text = Text::from(vec![Line::from(vec!["Value: ".into()])]);
                    Paragraph::new(counter_text)
                        .centered()
                        .block(block)
                        .render(area, buf);
                }
            }
        }
    }
}
