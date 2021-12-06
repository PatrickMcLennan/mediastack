# generate_presigned_url

Generates a presigned-url for PUT operations on an S3 bucket.

Example request:

```json
{
  "image_name": "new image name",
  "bucket_name": "existing bucket name"
}
```

Example output:

```json
{
  "status_code": 201,
  "message": "Created",
  "url": "https://new_presigned_url.com"
}
```

<hr />

## Compiling

### You need

- [Docker](https://www.docker.com/)

### Commands

- ```bash
  docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust sh -c "RUST_BACKTRACE=1 && rustup target add x86_64-unknown-linux-gnu && cargo build --target x86_64-unknown-linux-gnu --release";
  ```

Compile a production executable called `bootstrap` into `target/x86_64-unknown-linux-gnu/release`.

- `zip -r9 -j bootstrap.zip ./target/x86_64-unknown-linux-gnu/release/bootstrap`

Create a `.zip` of `bootstrap` to deploy to AWS

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
  --payload $(echo '{ image_name: "new image name", bucket_name: "existing bucket name" }' | base64 ) \
  test.json
  ```
  - Invokes the lambda, prints the output to `test.json` file.

docker run --rm --user "$(id -u)":"$(id -g)" -v "$PWD":/usr/src/myapp -w /usr/src/myapp rust cargo build --release
