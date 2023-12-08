#[cfg(feature = "axum")]
#[cfg(test)]
mod axum {
    use axum::{http::StatusCode, response::IntoResponse};
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
        #[body("some error message")]
        Blah(SomeStruct, String),
        Third,
        #[code(99)]
        Invalid,
    }

    #[test]
    fn check_error_codes() {
        let error = Error::First { _a: 1, _b: 2 };
        assert_eq!(StatusCode::BAD_REQUEST, error.into_response().status());

        let error = Error::Blah(SomeStruct { _a: 2, _b: 1 }, "something".to_owned());
        assert_eq!(StatusCode::NOT_FOUND, error.into_response().status());

        let error = Error::Third;
        assert_eq!(
            StatusCode::INTERNAL_SERVER_ERROR,
            error.into_response().status()
        );

        let error = Error::Invalid;
        assert_eq!(
            StatusCode::INTERNAL_SERVER_ERROR,
            error.into_response().status()
        );
    }
}

#[cfg(feature = "rocket")]
#[cfg(test)]
mod rocket_test {
    use error_http::ToResponse;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    struct SomeStruct {
        _a: i32,
        _b: u32,
    }

    #[derive(ToResponse)]
    enum Error {
        #[code(404)]
        #[body("message")]
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
    fn check_error_codes_rocket() {
        let rocket = rocket::build().mount("/", rocket::routes![error_internal, error_not_found]);
        let client = Client::new(rocket).expect("valid rocket instance");

        let response = client.get("/internal").dispatch();
        assert_eq!(response.status(), Status::InternalServerError);

        let response = client.get("/not_found").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
}

#[cfg(feature = "actix")]
#[cfg(test)]
mod actix_test {
    use actix_web::ResponseError;
    use error_http::ToResponse;

    #[derive(Debug)]
    struct SomeStruct {
        _a: i32,
        _b: u32,
    }

    #[derive(ToResponse, Debug)]
    enum Error {
        #[code(400)]
        First {
            _a: i32,
            _b: u32,
        },
        #[code(404)]
        #[body("some error message")]
        Blah(SomeStruct, String),
        Third,
        #[code(99)]
        Invalid,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("error")
        }
    }

    #[test]
    fn check_error_codes_actix() {
        let error = Error::First { _a: 1, _b: 2 };
        assert_eq!(
            actix_web::http::StatusCode::BAD_REQUEST,
            error.error_response().status()
        );

        let error = Error::Blah(SomeStruct { _a: 2, _b: 1 }, "something".to_owned());
        assert_eq!(
            actix_web::http::StatusCode::NOT_FOUND,
            error.error_response().status()
        );

        let error = Error::Third;
        assert_eq!(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            error.error_response().status()
        );

        let error = Error::Invalid;
        assert_eq!(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            error.error_response().status()
        );
    }
}
