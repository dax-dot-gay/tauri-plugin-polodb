use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

mod commands;

#[cfg(desktop)]
use desktop::Polodb;

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
    .invoke_handler(tauri::generate_handler![])
    .setup(|app, api| {
      #[cfg(desktop)]
      let polodb = desktop::init(app, api).unwrap();
      app.manage(polodb);
      Ok(())
    })
    .build()
}