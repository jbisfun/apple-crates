//! Filesystem used for bundle I/O: `std::fs` natively, isideload's VFS on wasm32
//! (where the bundle lives in memory and `std::fs` is unsupported).

#[cfg(target_arch = "wasm32")]
pub(crate) use isideload_vfs::fs;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use std::fs;

use std::path::Path;

/// `Path::exists` asks `std::fs`, so go through [`fs`] instead.
pub(crate) fn exists(path: &Path) -> bool {
    fs::metadata(path).is_ok()
}

/// `SystemTime::now()` panics on wasm32-unknown-unknown; read the JS clock there.
pub(crate) fn now() -> std::time::SystemTime {
    #[cfg(target_arch = "wasm32")]
    {
        let since_epoch = web_time::SystemTime::now()
            .duration_since(web_time::UNIX_EPOCH)
            .unwrap_or_default();
        std::time::UNIX_EPOCH + since_epoch
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        std::time::SystemTime::now()
    }
}
