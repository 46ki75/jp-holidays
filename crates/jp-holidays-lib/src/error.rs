#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("HTTP リクエスト中にエラーが発生: {0}")]
    Http(String),

    #[error("レスポンスボディの読み取りに失敗: {0}")]
    BodyRead(String),

    #[error("CSV のパースに失敗: {0}")]
    Parse(String),

    #[error("不正な日付: {0}")]
    InvalidDate(String),
}
