pub mod MainWindow {

    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::{
        buffer::Buffer,
        layout::{Alignment, Constraint, Direction, Layout, Rect},
        style::Stylize,
        text::{Line, Span},
        widgets::{block::Title, Block, Borders, Paragraph, Widget},
        Frame,
    };

    #[derive(Debug, Default)]
    pub struct MainWindow {
        pub exit: bool,
    }
    impl MainWindow {
        pub fn new() -> Self {
            MainWindow { exit: false }
        }
    }

    impl Widget for MainWindow {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let s = "
                                      ⢠⡄                                      
                                    ⢀⣠⣾⣷⣄⡀                                    
        ⣀⣀⣀⣀⣀                ⢀⣀⣤⣤⣄⠄⠈⠙⠻⣿⣿⠟⠋⠁⠄⣠⣤⣤⣀⡀                             
     ⢀⣴⣿⣿⠟⠋⠉⠉⠛⢶⣦          ⣤⣶⣿⣿⣿⠿⠿⠿⠗   ⠸⠇   ⠺⠿⠿⠿⣿⣿⣿⣶⣄⡀               ⠠⣤⣤⣶⣶⠋    
    ⣰⣿⣿⣿⠏      ⢻⠄⡀        ⠈⠛⠯⠅      ⣠⡶⠒⠒⢦⣆       ⠩⠝⠛⠁        ⢶⣶⣦⣀    ⢸⣿⣿⣿     
   ⢀⣿⣿⣿⡟       ⠈ ⠈⣿⣿⠗⠒⠒⢶⡀⠐⣶⣦⡀  ⢰⣶⣶ ⢸⣿⣧⡀  ⠛⠆⠐⣶⣶⠆  ⠰⣶⣶⠂  ⢲⣶⣶⠂  ⢸⣿⣿⣿⣷⣄  ⢸⣿⣿⣿     
   ⠘⣿⣿⣿⡇  ⠠⢤⣤⣤⣤⣤⡤⠄⣿⣿⠄⣀⣀⡀⠁ ⣿⣿⠻⣆ ⢸⣿⣿ ⠘⢿⣿⣿⣷⣤⡀  ⣿⣿⡀  ⢀⣿⣿  ⣀⣸⣿⣿   ⢸⣿⣿⣿ ⠙⢿⣦⣸⣿⣿⣿     
    ⢻⣿⣿⣿⡀   ⠘⣿⣿⣿⠄⢀⣯⣾⡿⠿⠿⠷⠄ ⣿⣿ ⠙⢷⣼⣿⣿   ⠉⠻⢿⣿⣿⣆ ⣿⣿⡿⠿⠿⢿⣿⣿⠠⠾⠿⢿⣿⣿⣦⡀ ⢸⣿⣿⣿   ⠉⢻⣿⣿⣿     
     ⠻⣿⣿⣷⡀   ⣿⣿⣿ ⠋⣿⣿   ⢀⡼⠄⣿⣿   ⠻⣿⣿ ⣧⡀   ⢹⣿⡿ ⣿⣿   ⢀⣿⣿   ⢸⣿⣿⠈⠙⠂⢸⣿⣿⣿    ⠘⣿⣿⣿     
      ⠈⠙⠛⠿⠷⠶⠚⠛⠉  ⠚⠛⠛⠛⠛⠛⠛⠃⠚⠛⠛⠓   ⠈⠻ ⠹⠿⠶⠤⠤⠾⠟⠁⡠⠿⠛⠓  ⠚⠛⠛⠒ ⠒⠛⠛⠛⠒⠂⠐⠛⠛⠛⠛⠓⠂   ⠈⠻⣿     
                  ⣀⣀⡀   ⣀⡀ ⢀⣀⡀   ⣀⣀⣀      ⣀      ⢀⣀⡀    ⣀⣀⣀⣀                  
                  ⢸⣿    ⠈⢿⣄⢻⣿    ⢸⣇⣸⠗    ⡘⢹⣧⠄   ⢰⣟⠁⠈⠃   ⠁⢸⡇                   
                  ⠼⠿⠄  ⠠⠤⠈⠃⠼⠿   ⠠⠼⠧⠄    ⠴⠄⠄⠿⠦⠄  ⠈⠻⠦⠄    ⠠⠼⠧⠄                  

";
            // Render logic for the other view widget

            let block = Block::default().borders(Borders::ALL);
            let inner_area = block.inner(area);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(inner_area);


            let paragraph = Paragraph::new(s).alignment(Alignment::Center);
            let other_string = "Welcome to Genshin Team Manager\r\nPress 't' to check your teams\r\nPress 'q' to exit the application";
            let other_paragraph = Paragraph::new(other_string).alignment(Alignment::Center);

            block.render(area, buf); // Renderizza il blocco esterno prima di renderizzare i paragrafi

            paragraph.render(chunks[0], buf);
            other_paragraph.render(chunks[1], buf);
        }
    }
}
