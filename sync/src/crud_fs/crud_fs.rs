use ethers::{utils, prelude::*};

use super::backend::BackendClient;
use super::store::StoreClient;
// use super::local::LocalClient;

/// A CRUD filesystem representation
/// # Fields
/// * `backend_client` - The backend client - this maintains FS state on a remote backend
/// * `store` - The store client - this maintains FS state on a remote store
/// # `local_client` - The local client - this maintains FS state on the local machine or filesystem
pub struct CrudFs {
    backend_client: BackendClient,
    store_client: StoreClient,
    // local_client: LocalClient,
}



