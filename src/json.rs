use serde_json::Value as Json;
use std::path::PathBuf;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

pub fn parse_packge_json(path: Option<&str>) -> Result<Json>
{
    let path: PathBuf = if let Some(str) = path {
        str.into()
    } else {
        "./package.json".into()
    };

    let source = std::fs::read_to_string(path)?;

    Ok(serde_json::from_str::<Json>(&source)?)
}

pub fn write_package_json() -> Result<()> { Ok(()) }

// let json = include_str!("../example_package_jsons/test1.json");

// let mut val: Value = serde_json::from_str(json).unwrap();

// let dependencies = val["dependencies"].as_object_mut().unwrap();
// dependencies.insert("react".to_owned(), Value::String("1.0.0".to_string()));

// // for (k, v) in dependencies.iter() {
// //     find_npm_package(k);
// // }

// search_npm_packages_by_term("nextjs");

// let result = serde_json::to_string_pretty(&val).unwrap();

// println!("Result: {}", result);
// let mut file = std::fs::File::create("out.json").unwrap();
// file.write_all(result.as_bytes()).unwrap();
//
