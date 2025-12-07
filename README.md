<img src="https://github.com/user-attachments/assets/da60e458-2992-413c-acb2-b7686b281776" alt="CurseForge logo" width="100" height="100" align="left" />

### *CurseForge API*
A complete Rust wrapper.

---

### API Key

You'll need a CurseForge API key to use this library, [request one here](https://console.curseforge.com/).

### Installation

```sh
cargo add curseforgeapi
```

### Usage

Create a `.env` file in your project root:

```env
CURSEFORGE_API_KEY='your-api-key-here'
```

Then use the client:

```rust
use curseforgeapi::{CurseForge, SearchModsParams};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("CURSEFORGE_API_KEY")?;

    let cf = CurseForge::new(&api_key);

    // Search for Minecraft mods
    let mods = cf.search_mods(&SearchModsParams {
        game_id: 432, // Minecraft
        search_filter: Some("jei".to_string()),
        ..Default::default()
    }).await?;

    for m in mods.data {
        println!("{} - {} downloads", m.name, m.download_count);
    }

    Ok(())
}
```

### Credit

- [CurseForge](https://curseforge.com/) for providing the API.
- [serde](https://github.com/serde-rs/serde) under the [MIT License](https://github.com/serde-rs/serde/blob/master/LICENSE-MIT), made
  by [Erick Tryzelaar](https://github.com/erickt), [David Tolnay](https://github.com/dtolnay),
  and [its contributors](https://github.com/serde-rs/serde/graphs/contributors).
- [reqwest](https://github.com/seanmonstar/reqwest) under the [MIT License](https://github.com/seanmonstar/reqwest/blob/master/LICENSE-MIT), made
  by [Sean McArthur](https://github.com/seanmonstar)
  and [its contributors](https://github.com/seanmonstar/reqwest/graphs/contributors).
- [Rust](https://github.com/rust-lang/rust) under the [MIT License](https://github.com/rust-lang/rust/blob/master/LICENSE-MIT), made
  by [its contributors](https://github.com/rust-lang/rust/graphs/contributors).

### License

MIT
