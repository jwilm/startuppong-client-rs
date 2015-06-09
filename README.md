startuppong-client-rs
=====================

[startuppong.com](http://www.startuppong.com) API wrapper

[![Circle CI](https://circleci.com/gh/jwilm/startuppong-client-rs.svg?style=svg)](https://circleci.com/gh/jwilm/startuppong-client-rs)

## About

The following methods of the startuppong.com API are supported:

- `/api/v1/get_players`
- `/api/v1/get_recent_matches_for_company`
- `/api/v1/add_match`

Each endpoint has a corresponding function published in the API. Data
returned from the API is strongly typed. Each resource type has a struct
associated with it.

Check out the [docs][] for more info.

## Cargo

Add the following to your Cargo.toml

```toml
[dependencies]
startuppong = "~0.1"
```

[docs]: http://www.jwilm.io/startuppong-client-rs/startuppong/

