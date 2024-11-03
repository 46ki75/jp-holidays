use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseList {
    pub results: Vec<crate::response::Response>,
}

impl From<Vec<crate::holiday::Holiday>> for ResponseList {
    fn from(holidays: Vec<crate::holiday::Holiday>) -> Self {
        let results = holidays
            .into_iter()
            .map(crate::response::Response::from)
            .collect();

        Self { results }
    }
}

impl ResponseList {
    /// path must end with a slash
    /// e.g. "./dist/api/v1/"
    pub fn save(&self, path: &str) -> Result<(), crate::error::Error> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(format!("{}list.json", path))?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}
