use crate::data::*;
use serde_json::Value as Json;

use super::PackageProvider;

pub struct NpmProvider {}

impl PackageProvider for NpmProvider
{
    fn get_package_endpoint(package_name: &str) -> String
    {
        format!("https://registry.npmjs.org/{}/latest", package_name)
    }

    fn get_package_endpoint_with_version(package_name: &str, version: crate::version::SemanticVersion) -> String
    {
        format!("https://registry.npmjs.org/{}/{}", package_name, version.to_string())
    }

    fn get_search_endpoint(term: &str) -> String
    {
        format!("https://registry.npmjs.com/-/v1/search?text={}&size=10", term)
    }
}

impl NpmProvider
{
    pub async fn search_npm_packages_by_term(term: &str) -> Vec<NpmPackage>
    {
        dbg!("Sending a request to npm!");

        if term.is_empty() {
            return vec![];
        }

        if term.is_empty() {}

        let endpoint = NpmProvider::get_search_endpoint(term);
        let result = reqwest::get(endpoint).await.unwrap();

        let text = result.text().await.unwrap();
        let json = serde_json::from_str::<Json>(&text).unwrap();

        json["objects"]
            .as_array()
            .unwrap()
            .iter()
            .map(|obj| NpmPackage::from_json_val(&obj["package"]))
            .collect()
    }

    pub async fn find_npm_package(package_name: &str) -> NpmPackage
    {
        let endpoint = NpmProvider::get_package_endpoint(package_name);
        let result = reqwest::get(endpoint).await.unwrap();
        let text = result.text().await.unwrap();

        serde_json::from_str::<NpmPackage>(&text).unwrap()
    }
}
