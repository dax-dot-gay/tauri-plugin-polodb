use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Polodb;
#[cfg(mobile)]
use mobile::Polodb;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the polodb APIs.
pub trait PolodbExt<R: Runtime> {
  fn polodb(&self) -> &Polodb<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PolodbExt<R> for T {
  fn polodb(&self) -> &Polodb<R> {
    self.state::<Polodb<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("polodb")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let polodb = mobile::init(app, api)?;
      #[cfg(desktop)]
      let polodb = desktop::init(app, api)?;
      app.manage(polodb);
      Ok(())
    })
    .build()
}
