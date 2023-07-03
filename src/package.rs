use std::fmt::{Display, Formatter, Result as FmtResult};



#[derive(Debug)]
struct Package {
    size: f32,
    repo: String,
    build: String, // there is no need for relatively complex date types
    desc: String,
}

impl Package {
    pub(crate) fn new(
        size: f32,
        repo: String,
        build: String,
        desc: String,
    ) -> Self {
        Self{ size, repo, build, desc }
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", self)
    }
}
