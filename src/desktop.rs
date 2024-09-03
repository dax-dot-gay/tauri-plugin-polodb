use serde::{de::DeserializeOwned, Serialize};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::daemon::messages::{PoloCommand, PoloManager};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Polodb<R>, ()> {
    Ok(Polodb {
        app: app.clone(),
        api: PoloManager::new(),
    })
}

/// Access to the polodb APIs.
pub struct Polodb<R: Runtime> {
    pub app: AppHandle<R>,
    pub api: PoloManager,
}

impl<R: Runtime> Polodb<R> {
    pub async fn call<T: Serialize + DeserializeOwned>(&self, command: PoloCommand) -> Result<T, crate::Error> {
        self.api.call::<T>(command).await
    }
}
