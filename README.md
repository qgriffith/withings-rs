# Withings-rs
Withings API Client in Rust

## About
First attempt at a rust project. This will Oauth to the Withings API and pull in your data.

## Use
You first need to create an Dev account on Withings to get a `client_id` and `client_secret` [Withings Dev Portal](https://developer.withings.com). Set the redirect URL to `http://localhost:8888` this client will bind to that on the machine it is ran on.

### Example use

```rust
use withings_rs::auth;
use std::env;

fn main () {
    println!("testing withings-rs");
    let client_id = env::var("WITHINGS_CLIENT_ID").unwrap();
    let client_secret = env::var("WITHINGS_CLIENT_SECRET").unwrap();
    let access_token = auth::get_access_code(client_id, client_secret);
    println!("Access token: {}", access_token);
}
```

## Disclaimer
This is very much a work in progress. Right now all it does is auth to the API.