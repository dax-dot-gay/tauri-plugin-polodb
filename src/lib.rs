use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;

mod commands;
mod daemon;
mod error;

use commands::{
    close_database, delete, delete_all, delete_one, find, find_all, find_one, insert, insert_one,
    list_databases, open_database, update, update_all, update_one, list_collections
};
pub use error::Error;

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
        .invoke_handler(tauri::generate_handler![
            list_databases,
            open_database,
            close_database,
            insert,
            insert_one,
            find,
            find_all,
            find_one,
            delete,
            delete_all,
            delete_one,
            update,
            update_all,
            update_one,
            list_collections
        ])
        .setup(|app, api| {
            #[cfg(desktop)]
            let polodb = desktop::init(app, api).unwrap();
            app.manage(polodb);
            Ok(())
        })
        .build()
}
