#[derive(Debug)]
pub struct App {
    pub args: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let args: Vec<String> = std::env::args()
            .skip(1)
            .collect();

        Self { args }
    }
}
