pub mod renderer{
    use std::io;

    use tui::{backend::CrosstermBackend, Terminal};

    pub fn render_gui(){
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
    }
}