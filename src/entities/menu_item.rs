pub mod menu_item{

    #[derive(Copy, Clone, Debug)]
    pub enum MenuItem{
        Home,
        Teams,
    }

    impl From<MenuItem> for usize{
        fn from(item: MenuItem) -> usize{
            match item{
                MenuItem::Home => 0,
                MenuItem::Teams => 1,
            }
        }
    }
}