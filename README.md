# error-http

The error-http crate defines a macro that implements proper HTTP responders for an enum with user-defined HTTP response codes. 

`#[derive(ToResponse)]` macro is web server orthogonal, which means for once defined enum only by switching feature
appropriate responder will be implemented for the chosen web server.

`#[code(XXX)]` defined for a given enum variant will result in `XXX` HTTP code being returned for it. Any variant without `#[code(XXX)]` will default to `500`.

## Supported web servers

This crate only allows choosing exactly one of avaliable implementations. Avaliable implementation:
- `axum`
- `rocket`

## Usage example

```rust
struct SomeStruct {
    a: i32,
    b: u32,
}

#[derive(ToResponse)]
enum Error {
    #[code(404)]
    First {
        a: i32,
        b: u32,
    },
    #[code(400)]
    Blah(SomeStruct, String),
    Third,
}


#[rocket::get("/internal")]
fn error_internal() -> Error {
    Error::Third
}

#[rocket::get("/not_found")]
fn error_not_found() -> Error {
    Error::Blah(SomeStruct { _a: 1, _b: 0 }, "something".to_owned())
}

#[test]
fn check_default_error_code() {
    let rocket = rocket::build().mount("/", rocket::routes![error_internal, error_not_found]);
    let client = Client::new(rocket).expect("valid rocket instance");

    let response = client.get("/internal").dispatch();
    assert_eq!(response.status(), Status::InternalServerError);

    let response = client.get("/not_found").dispatch();
    assert_eq!(response.status(), Status::NotFound);
}
```

## Future development
Possible exapnsion of this crate consists of:
- implementing `actix` support
- optional body for HTTP response
- tracing/logging
