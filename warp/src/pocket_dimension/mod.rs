pub mod query;

use crate::data::{DataObject, DataType};
use crate::error::Error;
use crate::sync::{Arc, Mutex, MutexGuard};
use crate::Extension;
use query::QueryBuilder;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub(super) type Result<T> = std::result::Result<T, Error>;

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[wasm_bindgen]
pub struct DimensionData(DimensionDataInner);

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionData(pub DimensionDataInner);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DimensionDataInner {
    Buffer { name: String, buffer: Vec<u8> },
    BufferNoFile { name: String, internal: Vec<u8> },
    Path { name: Option<String>, path: PathBuf },
}

impl<P: AsRef<std::path::Path>> From<P> for DimensionData {
    fn from(path: P) -> Self {
        let path = path.as_ref().to_path_buf();
        let name = path.file_name().map(|s| s.to_string_lossy().to_string());
        DimensionData(DimensionDataInner::Path { name, path })
    }
}

impl DimensionData {
    pub fn from_path(name: &str, path: &str) -> Self {
        DimensionData(DimensionDataInner::Path {
            name: Some(name.to_string()),
            path: std::path::PathBuf::from(path.to_string()),
        })
    }

    pub fn from_buffer(name: &str, buffer: &[u8]) -> Self {
        let name = name.to_string();
        let buffer = buffer.to_vec();
        DimensionData(DimensionDataInner::Buffer { name, buffer })
    }

    pub fn from_buffer_nofile(name: &str, internal: &[u8]) -> Self {
        let name = name.to_string();
        let internal = internal.to_vec();
        DimensionData(DimensionDataInner::BufferNoFile { name, internal })
    }
}

impl DimensionData {
    pub fn get_inner(&self) -> &DimensionDataInner {
        &self.0
    }
}

impl DimensionData {
    pub fn name(&self) -> Result<String> {
        match self.get_inner() {
            DimensionDataInner::Buffer { name, .. } => Ok(name.clone()),
            DimensionDataInner::BufferNoFile { name, .. } => Ok(name.clone()),
            DimensionDataInner::Path { name, .. } => name.clone().ok_or(Error::Other),
        }
    }
}

impl DimensionData {
    pub fn path(&self) -> Result<PathBuf> {
        match self.get_inner() {
            DimensionDataInner::Path { path, .. } => Ok(path.clone()),
            _ => Err(Error::Other),
        }
    }

    pub fn write_to_buffer(&self, buffer: &mut Vec<u8>) -> Result<()> {
        match self.get_inner() {
            DimensionDataInner::BufferNoFile { internal, .. } => {
                buffer.copy_from_slice(internal);
                return Ok(());
            }
            _ => {}
        }
        Err(Error::Other)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl DimensionData {
    pub fn write_from_path<W: Write>(&self, writer: &mut W) -> Result<()> {
        match self.get_inner() {
            DimensionDataInner::Path { name, path } if name.is_some() => {
                let mut file = std::fs::File::open(path)?;
                std::io::copy(&mut file, writer)?;
                return Ok(());
            }
            DimensionDataInner::BufferNoFile { internal, .. } => {
                let mut cursor = std::io::Cursor::new(internal);
                std::io::copy(&mut cursor, writer)?;
                return Ok(());
            }
            _ => {}
        }
        Err(Error::Other)
    }
}

/// PocketDimension interface will allow `Module` to store data for quick indexing and searching later on. This would be useful
/// for caching frequently used data so that request can be made faster. This makes it easy by sorting the data per module, as well
/// as allowing querying by specific information stored inside the payload of the `DataObject` for a quick turnaround for search
/// results.
pub trait PocketDimension: Extension + Send + Sync {
    /// Used to add data to `PocketDimension` for `Module`
    fn add_data(&mut self, dimension: DataType, data: &DataObject) -> Result<()>;

    /// Used to check to see if data exist within `PocketDimension`
    fn has_data(&mut self, dimension: DataType, query: &QueryBuilder) -> Result<()>;

    /// Used to obtain a list of `DataObject` for `Module`
    fn get_data(
        &self,
        dimension: DataType,
        query: Option<&QueryBuilder>,
    ) -> Result<Vec<DataObject>>;

    /// Returns the total size within the `Module`
    fn size(&self, dimension: DataType, query: Option<&QueryBuilder>) -> Result<i64>;

    /// Returns an total amount of `DataObject` for `Module`
    fn count(&self, dimension: DataType, query: Option<&QueryBuilder>) -> Result<i64>;

    /// Will flush out the data related to `Module`.
    fn empty(&mut self, dimension: DataType) -> Result<()>;
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub struct PocketDimensionAdapter {
    object: Arc<Mutex<Box<dyn PocketDimension>>>,
}

impl PocketDimensionAdapter {
    pub fn new(object: Arc<Mutex<Box<dyn PocketDimension>>>) -> Self {
        PocketDimensionAdapter { object }
    }

    pub fn inner(&self) -> Arc<Mutex<Box<dyn PocketDimension>>> {
        self.object.clone()
    }

    pub fn inner_guard(&self) -> MutexGuard<Box<dyn PocketDimension>> {
        self.object.lock()
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl PocketDimensionAdapter {
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn add_data(&mut self, dim: DataType, data: &DataObject) -> Result<()> {
        self.inner_guard().add_data(dim, data)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn has_data(&mut self, dim: DataType, query: &QueryBuilder) -> Result<()> {
        self.inner_guard().has_data(dim, query)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn size(&self, dim: DataType, query: Option<QueryBuilder>) -> Result<i64> {
        self.inner_guard().size(dim, query.as_ref())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn count(&self, dim: DataType, query: Option<QueryBuilder>) -> Result<i64> {
        self.inner_guard().count(dim, query.as_ref())
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn empty(&mut self, dimension: DataType) -> Result<()> {
        self.inner_guard().empty(dimension)
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn id(&self) -> String {
        self.inner_guard().id()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn name(&self) -> String {
        self.inner_guard().name()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn description(&self) -> String {
        self.inner_guard().description()
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
    pub fn module(&self) -> crate::module::Module {
        self.inner_guard().module()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl PocketDimensionAdapter {
    pub fn get_data(&self, dim: DataType, query: Option<QueryBuilder>) -> Result<Vec<DataObject>> {
        self.inner_guard().get_data(dim, query.as_ref())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl PocketDimensionAdapter {
    #[wasm_bindgen]
    pub fn get_data(&self, dim: DataType, query: Option<QueryBuilder>) -> Result<Vec<JsValue>> {
        self.inner_guard().get_data(dim, query.as_ref()).map(|s| {
            s.iter()
                .map(|i| serde_wasm_bindgen::to_value(&i).unwrap())
                .collect::<Vec<_>>()
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub mod ffi {
    use crate::data::{Data, DataType};
    use crate::pocket_dimension::query::QueryBuilder;
    use crate::pocket_dimension::PocketDimensionAdapter;

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_add_data(
        ctx: *mut PocketDimensionAdapter,
        dimension: DataType,
        data: *const Data,
    ) -> bool {
        if ctx.is_null() {
            return false;
        }

        if data.is_null() {
            return false;
        }

        let pd = &mut *ctx;
        let data = &*data;

        pd.inner_guard().add_data(dimension, data).is_ok()
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_has_data(
        ctx: *mut PocketDimensionAdapter,
        dimension: DataType,
        query: *const QueryBuilder,
    ) -> bool {
        if ctx.is_null() {
            return false;
        }

        if query.is_null() {
            return false;
        }

        let pd = &mut *ctx;
        let query = &*query;

        pd.inner_guard().has_data(dimension, query).is_ok()
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_get_data(
        ctx: *const PocketDimensionAdapter,
        dimension: DataType,
        query: *const QueryBuilder,
    ) -> *const Data {
        if ctx.is_null() {
            return std::ptr::null();
        }

        let query = match query.is_null() {
            true => None,
            false => Some(&*query),
        };

        let pd = &*ctx;

        match pd.inner_guard().get_data(dimension, query) {
            Ok(list) => {
                let list = std::mem::ManuallyDrop::new(list);
                list.as_ptr()
            }
            Err(_) => std::ptr::null(),
        }
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_size(
        ctx: *const PocketDimensionAdapter,
        dimension: DataType,
        query: *const QueryBuilder,
    ) -> i64 {
        if ctx.is_null() {
            return 0;
        }

        let query = match query.is_null() {
            true => None,
            false => Some(&*query),
        };

        let pd = &*ctx;

        pd.inner_guard().size(dimension, query).unwrap_or(0)
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_count(
        ctx: *const PocketDimensionAdapter,
        dimension: DataType,
        query: *const QueryBuilder,
    ) -> i64 {
        if ctx.is_null() {
            return 0;
        }

        let query = match query.is_null() {
            true => None,
            false => Some(&*query),
        };

        let pd = &*ctx;

        pd.inner_guard().count(dimension, query).unwrap_or(0)
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_empty(
        ctx: *mut PocketDimensionAdapter,
        dimension: DataType,
    ) -> bool {
        if ctx.is_null() {
            return false;
        }

        let pd = &mut *ctx;

        pd.inner_guard().empty(dimension).is_ok()
    }

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn pocket_dimension_free(ctx: *mut PocketDimensionAdapter) {
        if ctx.is_null() {
            return;
        }
        drop(Box::from_raw(ctx))
    }
}
