use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Response {
    pub name: String,
    pub date: String,
    pub public: bool,
    pub holiday: bool,
}
