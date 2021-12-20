# write_widescreen_wallpapers_to_dynamo

Makes HTTP call to front page of `/r/WidescreenWallpaper` and gets all posts. Filters out NSFW & Promoted images, compares against records currently in DynamoDB and then writes new images to a DynamoDB table.

Example DynamoDB record:

```json
{
  "created_at": {
    "N": 1638897941134
  },
  "name": {
    "S": "new image 1 name"
  },
  "pk": {
    "S": "new image 1 name"
  },
  "sk": {
    "S": "WidescreenWallpaper|new image 1 name"
  },
  "updated_at": {
    "N": 1638897941134
  },
  "media_type": {
    "S": "image"
  },
  "url": {
    "S": "new image 1 url"
  }
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
