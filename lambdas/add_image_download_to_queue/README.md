# add_image_download_to_queue

Interprets DynamoDB stream that gets triggered on any write operation. Any newly written record where `media_type = widescreen_wallpaper`, the following `QueueImage` struct is stringified & pushed on the `media-sqs-images` queue.

```json
{
  "name": "image name",
  "url": "image url"
}
```

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
