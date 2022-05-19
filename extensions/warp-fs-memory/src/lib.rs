pub mod item;

#[cfg(not(target_arch = "wasm32"))]
pub mod native;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use item::Item;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use warp::error::Error;

use warp::pocket_dimension::PocketDimension;
use warp::sync::{Arc, Mutex, MutexGuard};

use warp::constellation::directory::Directory;
use warp::hooks::Hooks;
use warp::module::Module;
use warp::Extension;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct MemorySystemInternal(item::directory::Directory);

impl AsRef<item::directory::Directory> for MemorySystemInternal {
    fn as_ref(&self) -> &item::directory::Directory {
        &self.0
    }
}

impl AsMut<item::directory::Directory> for MemorySystemInternal {
    fn as_mut(&mut self) -> &mut item::directory::Directory {
        &mut self.0
    }
}

impl Default for MemorySystemInternal {
    fn default() -> Self {
        MemorySystemInternal(item::directory::Directory::new("root"))
    }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct MemorySystem {
    index: Directory,
    current: Directory,
    path: PathBuf,
    modified: DateTime<Utc>,
    #[serde(skip)]
    internal: MemorySystemInternal,
    #[serde(skip)]
    cache: Option<Arc<Mutex<Box<dyn PocketDimension>>>>,
    #[serde(skip)]
    hooks: Option<Arc<Mutex<Hooks>>>,
}

impl Default for MemorySystem {
    fn default() -> Self {
        Self {
            index: Directory::new("root"),
            current: Directory::new("root"),
            path: PathBuf::new(),
            modified: Utc::now(),
            internal: MemorySystemInternal::default(),
            cache: None,
            hooks: None,
        }
    }
}

impl MemorySystem {
    pub fn new() -> Self {
        MemorySystem::default()
    }

    pub fn set_cache(&mut self, cache: Arc<Mutex<Box<dyn PocketDimension>>>) {
        self.cache = Some(cache);
    }

    pub fn set_hook(&mut self, hook: Arc<Mutex<Hooks>>) {
        self.hooks = Some(hook)
    }

    pub fn get_cache(&self) -> anyhow::Result<MutexGuard<Box<dyn PocketDimension>>> {
        let cache = self
            .cache
            .as_ref()
            .ok_or_else(|| anyhow!("Pocket Dimension Extension is not set"))?;

        let inner = cache.lock();
        Ok(inner)
    }
}

impl MemorySystemInternal {
    pub fn new() -> Self {
        MemorySystemInternal::default()
    }
}

impl Extension for MemorySystem {
    fn id(&self) -> String {
        String::from("warp-fs-memory")
    }
    fn name(&self) -> String {
        String::from("Basic In-Memory FileSystem")
    }

    fn description(&self) -> String {
        String::from("Basic In-Memory Filesystem extension")
    }
    fn module(&self) -> Module {
        Module::FileSystem
    }
}

pub mod ffi {
    use crate::MemorySystem;
    use std::ffi::c_void;
    use warp::constellation::ConstellationAdapter;
    use warp::sync::{Arc, Mutex};

    #[allow(clippy::missing_safety_doc)]
    #[no_mangle]
    pub unsafe extern "C" fn constellation_fs_memory_create_context() -> *mut c_void {
        let obj = Box::new(ConstellationAdapter::new(Arc::new(Mutex::new(Box::new(
            MemorySystem::new(),
        )))));
        Box::into_raw(obj) as *mut ConstellationAdapter as *mut c_void
    }
}
