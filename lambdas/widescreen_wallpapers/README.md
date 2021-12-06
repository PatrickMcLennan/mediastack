# widescreen_wallpapers

Makes an http call to the subreddit and returns a list of the `name` and `url` of all images on the front page. Any posts flagged as `nsfw` or `promoted` are filtered out.

Example output:

```json
{
  "status_code": 200,
  "body": [
    { "name": "picture_1", "url": "https://download_picture_1.com" },
    { "name": "picture_2", "url": "https://download_picture_2.com" }
  ]
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

Create a `.zip` of the bootstrap in to deploy to AWS

<hr />

## Developing

### You need

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/)

### Commands

- ```
  docker run --rm \
  -e DOCKER_LAMBDA_STAY_OPEN=1 -p 9001:9001 \
  -v "$PWD"/target/x86_64-unknown-linux-gnu/release/bootstrap:/var/task/bootstrap:ro,delegated \
  lambci/lambda:provided main
  ```
  - Starts a Docker container for your compiled lambda
- ```
  aws lambda invoke \
  --endpoint http://localhost:9001 \
  --no-sign-request --function-name=rust_lambda \
  --invocation-type=RequestResponse \
  --payload $(echo '{}' | base64 ) \
  test.json
  ```
  - Invokes the lambda, prints the output to `test.json` file.
