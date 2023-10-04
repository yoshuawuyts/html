use crate::Result;
use async_std::io::WriteExt;

pub async fn fetch() -> Result<()> {
    async fn fetch(from: &str, to: &str) -> Result<()> {
        eprintln!("fetching: {from}");
        let body = surf::get(from).recv_string().await?;
        let mut target = async_std::fs::File::create(to).await?;
        target.write_all(body.as_bytes()).await?;
        eprintln!("updated: {to}");
        Ok(())
    }
    eprintln!("task: fetch");
    fetch(super::HTML_STANDARD_URL, super::HTML_STANDARD_PATH).await?;
    fetch(super::ARIA_STANDARD_URL, super::ARIA_STANDARD_PATH).await?;
    fetch(
        super::HTML_ARIA_STANDARD_URL,
        super::HTML_ARIA_STANDARD_PATH,
    )
    .await?;
    Ok(())
}
