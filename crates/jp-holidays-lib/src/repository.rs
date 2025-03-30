// 国民の祝日 CSV 配布 URL
const ENDPOINT: &str = "https://www8.cao.go.jp/chosei/shukujitsu/syukujitsu.csv";

#[async_trait::async_trait]
pub trait HolidayRepository: Send + Sync {
    async fn fetch_csv(&self) -> Result<bytes::Bytes, crate::error::Error>;
}

pub struct HolidayRepositoryImpl;

#[async_trait::async_trait]
impl HolidayRepository for HolidayRepositoryImpl {
    async fn fetch_csv(&self) -> Result<bytes::Bytes, crate::error::Error> {
        let client = reqwest::Client::new();

        let response = client
            .get(ENDPOINT)
            .send()
            .await
            .map_err(|e| crate::error::Error::Http(e.to_string()))?;

        let response_bytes = response
            .bytes()
            .await
            .map_err(|e| crate::error::Error::BodyRead(e.to_string()))?;

        Ok(response_bytes)
    }
}

pub struct HolidayRepositoryStub;

#[async_trait::async_trait]
impl HolidayRepository for HolidayRepositoryStub {
    async fn fetch_csv(&self) -> Result<bytes::Bytes, crate::error::Error> {
        let response_bytes = include_bytes!("../tests/shift-jis.csv");

        Ok(bytes::Bytes::from_static(response_bytes))
    }
}
