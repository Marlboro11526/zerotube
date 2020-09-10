# ZeroTube

### Windows Setup For Local Development

Initial
- Get [mkcert](https://github.com/FiloSottile/mkcert)
- Run `mkcert 127.0.0.1`
- Rename the key one `key.pem` and the other `cert.pem`
- Move both of these to the backend dir
- Run `openssl x509 -outform der -in cert.pem -out cert.crt`
- Install `cert.crt` as a root cert

DB
- `cargo install diesel_cli --no-default-features --features sqlite`
- `diesel migration run`

Running
- `cargo run` in backend folder
- `npm run start` in frontend folder
