pub mod query;

use crate::query::QueryBuilder;
use warp_common::Result;
use warp_data::DataObject;
use warp_module::Module;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DimensionDataType {
    Json,
    String,
    Buffer,
}

/// PocketDimension interface will allow `Module` to store data for quick indexing and searching later on. This would be useful
/// for caching frequently used data so that request can be made faster. This makes it easy by sorting the data per module, as well
/// as allowing querying by specific information stored inside the payload of the `DataObject` for a quick turnaround for search
/// results.
pub trait PocketDimension {
    /// Used to add data to `PocketDimension` for `Module`
    fn add_data<I: Into<Module>>(&mut self, dimension: I, data: &DataObject) -> Result<()>;

    /// Used to check to see if data exist within `PocketDimension`
    fn has_data<I: Into<Module>>(&mut self, dimension: I, query: &QueryBuilder) -> Result<()>;

    /// Used to obtain a list of `DataObject` for `Module`
    fn get_data<I: Into<Module>>(
        &self,
        dimension: I,
        query: Option<&QueryBuilder>,
    ) -> Result<Vec<DataObject>>;

    /// Returns the total size within the `Module`
    fn size<I: Into<Module>>(&self, dimension: I, query: Option<&QueryBuilder>) -> Result<i64>;

    /// Returns an total amount of `DataObject` for `Module`
    fn count<I: Into<Module>>(&self, dimension: I, query: Option<&QueryBuilder>) -> Result<i64>;

    /// Will flush out the data related to `Module`.
    fn empty<I: Into<Module>>(&mut self, dimension: I) -> Result<()>;
}
