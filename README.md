# Space Track

A simple wrapper for the [Space-Track.org](https://www.space-track.org/) API.

## How to use

```rust
use space_track::{Config, Credentials, Direction, LaunchSiteField, SpaceTrack};

async fn main() -> Result<(), Box<dyn Error>> {
  let mut space_tracker = SpaceTrack::new(Credentials {
      identity: "name@example.com".to_string(),
      password: "my-password".to_string(),
  });

  // get first 10 launch sites, sorted by the site code in descending order
  let launch_sites = space_track
      .launch_site(
          Config::new()
              .limit(10)
              .offset(0)
              .order_by(LaunchSiteField::SiteCode, Direction::Descending)
              .distinct(),
      )
      .await?;

  for launch_site in launch_sites {
      println!("Launch site code: {}", launch_site.site_code);
  }
}
```

Obtain credentials from [Space-Track website](https://www.space-track.org/). Please respect the API Use Guidelines described in the [API documentation](https://www.space-track.org/documentation#/api).

## API

The full API documentation can be on the [Space-Track.org website](https://www.space-track.org/documentation#/api).

Every method name maps to a "Request Class" from the documentation:

```rust
let decay = space_track.decay(Config::new()).await?;
```

The config object contains additional configuration for the API call, such as limit, offset, sort. By default, it is created with a limit of 100, which is an intentional design decision as to prevent unnecessary load on the API.

### Empty config

There is an `empty` config constructor, which is created with no limit. Use with caution:

```rust
let decay = space_track.decay(Config::empty()).await?;
```

Every method has an `all_` prefixed version, which requires no config and is created with an empty one. The following is equivalent to the above code:

```rust
let decay = space_track.all_decay().await?;
```

## Config

The configuration object follows the builder pattern and has the following properties:

- `limit` - amount of records to retrieve
- `offset` - amount of records to skip
- `order_by` - field and direction to sort by
- `distinct` - boolean, indicating whether to return distinct records

```rust
Config::new()
  .limit(10)
  .offset(0)
  .order_by(LaunchSiteField::SiteCode, Direction::Descending)
  .distinct()
```

## Why `mut`?

The short version is that the instance saves and refreshes its credentials seamlessly, but the trade-off is that every method may potentially mutate itself.

## What it does

### Authentication

The Space Track API authentication works by logging in to the website and using the resulting cookie, which is good for around 2 hours. In the wrapper, that authentication happens seamlessly if there is no cookie, or if the cookie has expired.

The cookie is saved in the `SpaceTrack` instance, so whenever a request is made, the cookie may need to be refreshed. That's why every call on the instance can potentially mutate itself. The trade-off is that everything happens under the hood, but the instance has to be mutable, which may be annoying if you need to borrow it multiple times.

The plan for the future is to have a version of the wrapper that works with associated functions only, but where cookies have to be managed by the user.

### Deserialization

The wrapper handles deserialization, which doesn't sound like much, until you take a look at the responses that the API returns: the "numbers" are all strings, a lot of fields are not marked as optional, but actually are, the casing is all UPPERCASE, some words are written as one, there are fields which are `enum(Y, N)`, which have been converted to booleans, etc. All that has been handled so that the structs returned by the API look and feel normal.

The dates have been left untouched _for now_.
The numbers defined in the model definition on the API often don't reflect the reality, so currently number types have been defined on the safe side, but the plan is to refine them further.

## Contribution

I am a Rust newcomer, so there may be a lot of glaring issues that I am not aware of. For such general improvements, feel free to just open a pull request.

There are a couple of areas that need improvement (such as tightening number types). The progress for those will be tracked in GitHub issues. If you wish to contribute, feel free to take a look there.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
