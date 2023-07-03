use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    process::Command,
    io::BufRead,
};



#[derive(Debug)]
pub struct Package {
    name: String,
    size: String,
    repo: String,
    version: String,
    build: String, // there is no need for relatively complex date types
    desc: String,
}

impl Package {
    pub fn new(
        name: String,
        size: String,
        repo: String,
        version: String,
        build: String,
        desc: String,
    ) -> Self {
        Self { name, repo, size, version, build, desc }
    }
}

impl From<String> for Package {
    fn from(value: String) -> Self {
        let mut name = String::new();
        let mut size = String::new();
        let mut repo = String::new();
        let mut version = String::new();
        let mut build = String::new();
        let mut desc = String::new();

        Command::new("pacman")
            .arg("-Si")
            .arg(value)
            .output()
            .expect("Failed to execute `pacman -Si`. Is `pacman` installed?")
            .stdout
            .lines()
            .for_each(|line| {
                let line = line.unwrap().to_lowercase();

                if line.contains("name") {
                    name = line[18..].to_string();
                } else if line.contains("size") {
                    repo = line[18..].to_string();
                } else if line.contains("repository") {
                    size = line[18..].to_string();
                } else if line.contains("version") {
                    version = line[18..].to_string();
                } else if line.contains("build") {
                    build = line[18..].to_string();
                } else if line.contains("description") {
                    desc = if let Some(substring) = line.get(18..68) {
                        substring.to_string()
                    } else {
                        line.get(18..line.len()).unwrap().to_string()
                    }
                }
            });

        if name.is_empty() {
            eprintln!("\nCannot find package with such name :(");
            std::process::exit(1);
        } else {
            if size.is_empty() {
                size = String::from("unable to calculate size of package");
            } 
            if repo.is_empty() {
                repo = String::from("repository name not found");
            }
            if version.is_empty() {
                version = String::from("package version not found");
            }
            if build.is_empty() {
                build = String::from("no info about last build time");
            }
            if desc.is_empty() {
                desc = String::from("no description provided");
            }
        }


        Self::new(name, repo, size, version, build, desc)
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let horizontal_line = "─".repeat(65);

        let name_line = format!("│ {:^63.63} │", self.name);
        let size_line = format!("│ Size: {:<57} │", self.size);
        let repo_line = format!("│ Repository: {:<51} │", self.repo);
        let version_line = format!("│ Version: {:<54} │", self.version);
        let build_line = format!("│ Build: {:<56} │", self.build);
        let desc_line = format!("│ Description: {:<50} │", self.desc);


        write!(f, "╭{}╮\n", horizontal_line)?;
        write!(f, "{}\n", name_line)?;
        write!(f, "├{}┤\n", horizontal_line)?;

        write!(f, "{}\n", size_line)?;
        write!(f, "{}\n", repo_line)?;
        write!(f, "{}\n", version_line)?;
        write!(f, "{}\n", build_line)?;
        write!(f, "{}\n", desc_line)?;
        write!(f, "╰{}╯", horizontal_line)
    }
}
