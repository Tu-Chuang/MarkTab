use std::fs;
use std::io::Write;
use std::path::Path;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    pub version: String,
    pub version_code: i32,
    pub download_url: String,
    pub changelog: String,
}

pub struct UpgradeService;

impl UpgradeService {
    pub async fn check_update() -> Result<Option<VersionInfo>, AppError> {
        let client = Client::new();
        let resp = client.get("https://api.MARKTAB.cc/version")
            .send()
            .await?
            .json::<VersionInfo>()
            .await?;

        if resp.version_code > crate::APP_VERSION_CODE {
            Ok(Some(resp))
        } else {
            Ok(None)
        }
    }

    pub async fn download_update(version: &VersionInfo) -> Result<(), AppError> {
        let client = Client::new();
        let response = client.get(&version.download_url)
            .send()
            .await?;

        let tmp_path = Path::new("tmp");
        if !tmp_path.exists() {
            fs::create_dir(tmp_path)?;
        }

        let mut file = fs::File::create("tmp/update.zip")?;
        let content = response.bytes().await?;
        file.write_all(&content)?;

        // 解压更新包
        let file = fs::File::open("tmp/update.zip")?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            if file.is_dir() {
                fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        fs::create_dir_all(p)?;
                    }
                }
                let mut outfile = fs::File::create(&outpath)?;
                std::io::copy(&mut file, &mut outfile)?;
            }
        }

        // 清理临时文件
        fs::remove_dir_all(tmp_path)?;

        Ok(())
    }
} 