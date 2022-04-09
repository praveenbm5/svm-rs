use reqwest::get;
use semver::Version;
use serde::{
    de::{self, Deserializer},
    Deserialize, Serialize,
};
use std::collections::BTreeMap;
use url::Url;

use super::Releases;
use crate::{error::SolcVmError, platform::platform};

static URL_PREFIX: &str =
    "https://github.com/nikitastupin/solc/raw/08e633c3585e2a053f0662714098440e639a074a/linux/aarch64";

static RELEASES: Lazy<Releases> = Lazy::new(|| {
    serde_json::from_str(include_str!("../list/linux-aarch64.json"))
        .expect("could not parse list linux-aarch64.json")
});

#[cfg(feature = "blocking")]
pub fn blocking_all_releases() -> Result<Releases, SolcVmError> {
    Ok(RELEASES.clone())
}

pub async fn all_releases() -> Result<Releases, SolcVmError> {
    Ok(RELEASES.clone())
}

pub fn artifact_url(version: &Version, artifact: &str) -> Result<Url, SolcVmError> {
    if RELEASES.releases.contains_key(version) {
        Ok(Url::parse(&format!("{}/{}", URL_PREFIX, artifact))?)
    } else {
        Err(SolcVmError::UnsupportedVersion(
            version.to_string(),
            platform().to_string(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_aarch64() {
        assert_eq!(RELEASES.releases.len(), 45);
        assert_eq!(RELEASES.builds.len(), 45);
    }

    #[tokio::test]
    async fn test_all_releases_linux_aarch64() {
        assert!(all_releases().await.is_ok());
    }

    #[tokio::test]
    async fn releases_roundtrip() {
        let releases = all_releases().await.unwrap();
        let s = serde_json::to_string(&releases).unwrap();
        let de_releases: Releases = serde_json::from_str(&s).unwrap();
        assert_eq!(releases, de_releases);
    }
}
