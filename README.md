# tauri-plugin-polodb
A Tauri plugin to expose the PoloDB embedded database to applications

## Installation

To install the Rust plugin, run:

```bash
cargo add tauri-plugin-polodb
```

Then, load the plugin in your Tauri app as follows:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_polodb::init()) // THIS LINE
        // More builder methods
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Finally, install the JS client bindings:

```bash
yarn add tauri-plugin-polodb-api
```

## Usage

In the Rust backend, all the PoloDB APIs can be accessed through the `AppHandle` as follows:

```rust
...
app.polodb().<methods>
...
```

On the client:

```typescript
import {Database} from "tauri-plugin-polodb-api";

const db = await Database.open("example", "./data"); // Open a database
const collection = db.collection<{[key: string]: any}>("example_collection"); // Get a reference to a collection
console.log(await collection.find_all()); // Returns all records in the collection
```

For query syntax, reference the PoloDB documentation.