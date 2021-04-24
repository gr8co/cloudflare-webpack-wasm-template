# Cloudflare Webpack WASM template

- Open with VSCode using the .devcontainer for the fastest way to start programming
- run `wrangler preview` (optionally, provide your `CF_ACCOUNT_ID` in ENV or `account_id` in wrangler.toml)

This project template showcases how you can call javascript functions within Rust through wasm-bindgen, 
and how to use Cloudflare workers with WASM through webpack.

Use as a wrangler template:
```console
wrangler generate $NAME https://github.com/ronaldslc/cloudflare-webpack-wasm-template
```