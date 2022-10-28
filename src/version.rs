use std::fmt::Display;

use crate::logging::debug_d;

#[derive(Debug, Copy, Clone, Default)]
pub struct SemanticVersion
{
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl SemanticVersion
{
    pub fn new(major: u8, minor: u8, patch: u8) -> Self { SemanticVersion { major, minor, patch } }
}

impl Into<String> for SemanticVersion
{
    fn into(self) -> String { return self.to_string(); }
}

impl Display for SemanticVersion
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl From<&str> for SemanticVersion
{
    fn from(version: &str) -> Self
    {
        let mut result: [u8; 3] = [0, 0, 0];
        let mut index = 0;

        for sub_str in version.split('.') {
            if index > 2 {
                break;
            }
            let mut is_numeric = true;

            for c in sub_str.chars() {
                if !c.is_numeric() {
                    is_numeric = false;
                }
            }

            if is_numeric {
                let as_num = u8::from_str_radix(sub_str, 10).unwrap_or(0);
                result[index] = as_num;
            } else {
                debug_d!("TODO: Handle non numeric version... {}", sub_str);
            }

            index += 1;
        }

        SemanticVersion::new(result[0], result[1], result[2])
    }
}

impl From<String> for SemanticVersion
{
    fn from(version: String) -> Self { return version.as_str().into(); }
}
