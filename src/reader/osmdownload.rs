use crate::reader::osmelements::Bbox;
use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

pub async fn download_bbox(path: &str, bbox: &Bbox) -> Result<()> {
    let target = format!(
        "https://api.openstreetmap.org/api/0.6/map?{}",
        bbox.to_string()
    );
    let response = reqwest::get(target).await?;
    let content = response.text().await?;
    let mut dest = File::create(path)?;
    copy(&mut content.as_bytes(), &mut dest)?;
    Ok(())
}
