# error-http

The error-http crate defines a macro that implements proper HTTP responders for an enum with user-defined HTTP response codes and error messages. 

`#[derive(ToResponse)]` macro is web server orthogonal, which means for once defined enum only by switching feature
appropriate responder will be implemented for the chosen web server.

`#[code(XXX)]` defined for a given enum variant will result in `XXX` HTTP code being returned for it. Any variant without `#[code(XXX)]` will default to `500`. Any invalid HTTP error code will default to `500`.

`#[body(message)]` defined for a given variant will add a body to the response. `body` can be any expression that 
produces `String` or `&str`. Currently, there is no option to change the content type.

## Supported web servers

This crate only allows choosing exactly one of avaliable implementations. Avaliable implementation:
- `actix-web` 
- `axum`
- `rocket`

## Usage example

```rust
use error_http::ToResponse;
struct SomeStruct {
    _a: i32,
    _b: u32,
}

#[derive(ToResponse)]
enum Error {
    #[code(400)]
    First {
        _a: i32,
        _b: u32,
    },
    #[code(404)]
    #[body("custom error message")]
    Blah(SomeStruct, String),
    Third,
    #[code(99)]
    Invalid,
}
```

## Future development
Possible expansion of this crate consists of:
- content type and JSON body
- tracing/logging
