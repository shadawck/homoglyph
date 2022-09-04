#[macro_use]
extern crate rocket;
extern crate rocket_cors;

use rocket::http::Method::{Get, Post};
use rocket::{fs::relative, fs::FileServer, serde::json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde::{Deserialize, Serialize};
use service::*;

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://127.0.0.1:8000",
        "http://127.0.0.1:8000/api/homoglyphs",
        "http://127.0.0.1:3000/api/homoglyphs",
        "http://127.0.0.1:3000",
        "http://localhost:3000",
        "http://localhost:3000/api/homoglyphs",
    ]);
    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Post, Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            //"Authorization",
            //"Accept",
            "Access-Control-Allow-Origin",
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap()
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
struct HomoglyphsRequest {
    sentence: String,
    n_permutation: Option<usize>,
    n_confusable: Option<usize>,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
struct HomoglyphsResponse {
    homoglyphs: Vec<Vec<String>>,
}

#[macro_export]
macro_rules! conv_json {
    ( $x:expr ) => {
        Json(HomoglyphsResponse {
            homoglyphs: homoglyphs_to_string($x),
        })
    };
}

#[get("/healthy")]
fn healthy() -> &'static str {
    "true"
}

#[post("/homoglyphs", data = "<homoglyphs_request>")]
fn homoglyphs(homoglyphs_request: Json<HomoglyphsRequest>) -> Json<HomoglyphsResponse> {
    println!("{:#?}", homoglyphs_request);
    let mut ch = ComputeHomoglyphs::new();

    if homoglyphs_request.n_permutation.is_some() && homoglyphs_request.n_confusable.is_some() {
        println!("both");

        return conv_json!(ch.compute_with_limit(
            &homoglyphs_request.sentence,
            homoglyphs_request.n_permutation.unwrap(),
            homoglyphs_request.n_confusable.unwrap(),
        ));
    } else if homoglyphs_request.n_permutation.is_some()
        && homoglyphs_request.n_confusable.is_none()
    {
        return conv_json!(ch.compute_with_n_permutation(
            &homoglyphs_request.sentence,
            homoglyphs_request.n_permutation.unwrap(),
        ));
    } else if homoglyphs_request.n_permutation.is_none()
        && homoglyphs_request.n_confusable.is_some()
    {
        {
            return conv_json!(ch.compute_with_n_confusable(
                &homoglyphs_request.sentence,
                homoglyphs_request.n_confusable.unwrap(),
            ));
        }
    } else {
        let default_confusable_limit = 8;
        let default_homoglyphs_limit = 1000;

        return conv_json!(ch.compute_with_limit(
            &homoglyphs_request.sentence,
            default_homoglyphs_limit,
            default_confusable_limit,
        ));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from(relative!("../client/build")))
        .mount("/api", routes![homoglyphs, healthy])
        .attach(make_cors())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homoglyphs_with_limit() {
        let homoglyphs_request = HomoglyphsRequest {
            sentence: "rust is best".to_string(),
            n_permutation: Some(10),
            n_confusable: Some(2),
        };

        use rocket::local::blocking::Client;
        let client = Client::tracked(super::rocket()).unwrap();
        let homoglyphs = client
            .post("/api/homoglyphs")
            .json(&homoglyphs_request)
            .dispatch();

        homoglyphs.into_json::<HomoglyphsResponse>().unwrap();
    }

    #[test]
    fn test_homoglyphs_with_n_permutation() {
        let homoglyphs_request = HomoglyphsRequest {
            sentence: "rust is best".to_string(),
            n_permutation: Some(10),
            n_confusable: None,
        };

        use rocket::local::blocking::Client;
        let client = Client::tracked(super::rocket()).unwrap();
        let homoglyphs = client
            .post("/api/homoglyphs")
            .json(&homoglyphs_request)
            .dispatch();

        homoglyphs.into_json::<HomoglyphsResponse>().unwrap();
    }

    #[test]
    fn test_homoglyphs_with_n_confusable() {
        let homoglyphs_request = HomoglyphsRequest {
            sentence: "rust is best".to_string(),
            n_permutation: None,
            n_confusable: Some(10),
        };

        use rocket::local::blocking::Client;
        let client = Client::tracked(super::rocket()).unwrap();
        let homoglyphs = client
            .post("/api/homoglyphs")
            .json(&homoglyphs_request)
            .dispatch();

        homoglyphs.into_json::<HomoglyphsResponse>().unwrap();
    }

    #[test]
    fn test_homoglyphs_with_default() {
        let homoglyphs_request = HomoglyphsRequest {
            sentence: "rust is best".to_string(),
            n_permutation: None,
            n_confusable: None,
        };

        use rocket::local::blocking::Client;
        let client = Client::tracked(super::rocket()).unwrap();
        let homoglyphs = client
            .post("/api/homoglyphs")
            .json(&homoglyphs_request)
            .dispatch();

        homoglyphs.into_json::<HomoglyphsResponse>().unwrap();
    }
}
