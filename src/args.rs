#[derive(Debug)]
struct App {
    args: Vec<String>,
}

impl App {
    fn new() -> Self {
        let args: Vec<String> = std::env::args().collect();

        Self { args }
    }
}
