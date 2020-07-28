use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use yew::callback::Callback;
use yew::format::{Json, Nothing, Text, Binary};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::util::Error;

enum ResponseType {
    JSON,
    TEXT,
    TOML,
    BINARY,
}

enum RequestMethod {
    GET,
    POST,
}

fn get<T>(url: &str, callback: Callback<Result<T, Error>>,) -> FetchTask
where for<'de> T: Deserialize<'de> + 'static + Debug {
    let handler = move |response: Response<Text>| {
        if let (meta, Ok(data)) = response.into_parts() {
            dbg!("Response: {:?}", &data);
            if meta.status.is_success() {
                let data: Result<T, _> = serde_json::from_str(&data);
                match data {
                    Ok(d) => callback.emit(Ok(d)),
                    Err(e) => {
                        dbg!("err: {:?}", e);
                        callback.emit(Err(Error::DeserializeError))
                    }
                }
            } else {
                match meta.status.as_u16() {
                    401 => callback.emit(Err(Error::Unauthorized)),
                    403 => callback.emit(Err(Error::Forbidden)),
                    404 => callback.emit(Err(Error::NotFound)),
                    500 => callback.emit(Err(Error::InternalServerError)),
                    // 422 => {
                    //     let data: Result<ErrorInfo, _> = serde_json::from_str(&data);
                    //     if let Ok(data) = data {
                    //         callback.emit(Err(Error::UnprocessableEntity(data)))
                    //     } else {
                    //         callback.emit(Err(Error::DeserializeError))
                    //     }
                    // }
                    _ => callback.emit(Err(Error::RequestError)),
                }
            }
        } else {
            callback.emit(Err(Error::RequestError))
        }
    };

    let request = Request::get(url).body(Nothing).unwrap();
    FetchService::fetch(request, handler.into()).unwrap()
}

fn fetch_binary<T>(method: RequestMethod, url :&str, body: impl Into<Text>) -> FetchTask
where for<'de> T: Deserialize<'de> + 'static + Debug {
    let callback = move |response: Response<Json<Result<T, _>>>| {
            let (meta, Json(data)) = response.into_parts();
            println!("META: {:?}, {:?}", meta, data);
            if meta.status.is_success() {
                println!("success");
            } else {
                println!("failed: {}", meta.status.as_u16());
            }
        };
    // let request = Request::get("/data.json").body(Nothing).unwrap();
    let builder = match method {
        GET => Request::get(url),
        POST => Request::post(url),
    };
    let request = builder.body(body).unwrap();
    // FetchService::fetch_binary(request, callback.into()).unwrap()
    FetchService::fetch(request, callback.into()).unwrap()
}
