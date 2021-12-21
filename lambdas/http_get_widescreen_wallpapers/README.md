# http_get_widescreen_wallpapers

Lambda scans DynamoDB for secondary index `media_type = widescreen_wallpaper` and returns all records in an HTTP response.

Ex. 200 response:

```json
{
  "status_code": 200,
  "images": [
    { "name": "image 1", "url": "https://image1.com" }
    { "name": "image 2", "url": "https://image2.com" }
  ],
  "message": "2 images found"
}
```

Ex. 404 response:

```json
{
  "status_code": 404,
  "images": [],
  "message": "No images were found."
}
```

Ex. 500 response:

```json
{
  "status_code": 500,
  "images": [],
  "message": "Error querying the the table, try again later."
}
```

<hr />

## Compiling

### You need

- [cross](https://github.com/rust-embedded/cross)
- rustup target [`x86_64-unknown-linux-gnu`](https://rust-lang.github.io/rustup/cross-compilation.html)

### Commands

- `cross build --target x86_64-unknown-linux-gnu --release`

Compile a production executable called `bootstrap` into `target/x86_64-unknown-linux-gnu/release`.

- `zip -r9 -j bootstrap.zip ./target/x86_64-unknown-linux-gnu/release/bootstrap`

Create a `.zip` of `bootstrap` to deploy to AWS

<hr />

## Developing

### You need

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/)
