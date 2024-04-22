pub mod teams_home_window {
    use ratatui::{
        buffer::Buffer,
        layout::Rect,
        widgets::{Block, Borders, Widget},
    };

    pub struct teams_home_window {
        pub exit: bool,
    }
    impl teams_home_window {
        pub fn new() -> Self {
            teams_home_window { exit: false }
        }
    }

    impl Widget for teams_home_window {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let test = "test";
            let block = Block::default().borders(Borders::ALL);
            let inner_area = block.inner(area);
            //render it
            block.render(area, buf);
        }
    }
}
