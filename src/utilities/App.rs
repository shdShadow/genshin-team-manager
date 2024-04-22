pub mod app {

    use std::{default, io};

    use crate::utilities::tui;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::{
        buffer::Buffer,
        layout::{Alignment, Layout, Rect},
        style::Stylize,
        symbols::border,
        text::{Line, Text},
        widgets::{
            block::{Position, Title},
            Block, Borders, Paragraph, Widget,
        },
        Frame,
    };
    use crate::utilities::MainWindow::MainWindow::MainWindow;
    use crate::utilities::teams_home_window::teams_home_window;
    #[derive(Debug, Default)]
    pub enum AppState {
        #[default]
        MainWindow,
        TeamsHome,
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
                KeyCode::Char('t') => {
                    self.state = AppState::TeamsHome;
                } 
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
                    let mainWindow = MainWindow::new();
                    mainWindow.render(area, buf);
                },
                AppState::TeamsHome => {
                    let teamsHome = teams_home_window::teams_home_window::new();
                    teamsHome.render(area, buf);
                }

            }
        }
    }
}
