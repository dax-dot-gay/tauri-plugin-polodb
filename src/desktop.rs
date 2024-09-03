use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> Result<Polodb<R>, ()> {
  Ok(Polodb(app.clone()))
}

/// Access to the polodb APIs.
pub struct Polodb<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Polodb<R> {
  
}
