use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LicenseId(pub String);

impl std::fmt::Display for LicenseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.as_ref().fmt(f)
    }
}

impl AsRef<String> for LicenseId {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for LicenseId {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl From<String> for LicenseId {
    fn from(s: String) -> LicenseId {
        LicenseId(s)
    }
}
