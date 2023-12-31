pub mod cargo_data;
pub mod license_data;
pub mod license_exception_id;
pub mod license_expression_lexer;
pub mod license_expression_parser;
pub mod license_id;
pub mod license_provider;
pub mod store;

pub use license_data::LicenseExceptionList;
pub use license_data::LicenseExceptionSummary;
pub use license_data::LicenseList;
pub use license_data::LicenseSummary;
pub use license_exception_id::LicenseExceptionId;
pub use license_expression_parser::ast;
pub use license_id::LicenseId;

use std::fmt::Debug;
use std::hash::Hash;

pub trait Store<K, V>: Debug + Send + Sync
where
    K: Send + Sync + Eq + Hash,
    V: Clone + Send + Sync,
{
    fn get(&self, key: &K) -> anyhow::Result<Option<V>>;
    fn insert(&self, key: K, val: V) -> anyhow::Result<()>;
    fn contains_key(&self, key: &K) -> anyhow::Result<bool> {
        Ok(self.get(key)?.is_some())
    }
}

trait LicenseProvider {
    fn get_license(
        &self,
        license_id: &LicenseId,
    ) -> anyhow::Result<Option<std::sync::Arc<serde_json::Value>>>;

    fn get_license_exception(
        &self,
        license_exception_id: &LicenseExceptionId,
    ) -> anyhow::Result<Option<std::sync::Arc<serde_json::Value>>>;
}
