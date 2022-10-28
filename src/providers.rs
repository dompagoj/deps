use crate::version::SemanticVersion;

pub mod npm_provider;

pub use npm_provider::*;

pub trait PackageProvider
{
    fn get_package_endpoint(package_name: &str) -> String;
    fn get_package_endpoint_with_version(package_name: &str, version: SemanticVersion) -> String;
    fn get_search_endpoint(term: &str) -> String;
}
