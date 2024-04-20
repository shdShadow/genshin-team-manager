use std::io;

pub mod utilities;
pub mod entities;
pub mod fileManager;
fn main() -> io::Result<()>{

     let mut terminal = utilities::tui::init()?;
    let app_result = utilities::App::app::App::default().run(&mut terminal);
    utilities::tui::restore()?;
    app_result
}
