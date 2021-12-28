# http_grant_api_key

Takes in email + password, and if valid, returns the API key needed for API gateway.

Ex. Request:

```json
{
  "email": "an_email@email.com",
  "password": "a password"
}
```
Ex. 200 Response:

```json
{
  "status_code": 200,
  "body": API_GATEWAY_API_KEY
}
```

Ex. 400 Response:

```json
{
  "status_code": 400,
  "body": "Invalid parameters"
}
```


><br />**Important!**<br /> <br />For now, everything is hardcoded (see `.env.example`).  Only the `ADMIN_EMAIL` and `ADMIN_PASSWORD` will be accepted, and this lambda will always return the `API_GATEWAY_API_KEY`.  Undetermined if this will be updated.  The onus of key rotation & security will fall on the User for the time being.<br /><br />

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
