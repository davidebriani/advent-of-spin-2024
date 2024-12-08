use spin_sdk::{
    http::{IntoResponse, Method, Request, Response},
    http_component,
    key_value::Store,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Wishlist {
    name: String,
    items: Vec<String>,
}

#[http_component]
fn handle_request(req: Request) -> anyhow::Result<impl IntoResponse> {
    let store = Store::open_default()?;

    let (status, body) = match *req.method() {
        Method::Post => {
            let wishlist: Wishlist = match serde_json::from_slice(req.body()) {
                Ok(wishlist) => wishlist,
                Err(_) => {
                    return Ok(Response::builder()
                        .status(400)
                        .header("Content-Type", "application/json")
                        .body(Some("Invalid JSON".as_bytes().to_vec()))
                        .build());
                }
            };

            let mut wishlists: Vec<Wishlist> = match store.get("wishlists")? {
                Some(value) => serde_json::from_slice(&value).unwrap_or_else(|_| Vec::new()),
                None => Vec::new(),
            };

            wishlists.push(wishlist);
            store.set("wishlists", &serde_json::to_vec(&wishlists)?)?;

            let response_body =
                serde_json::to_string(&wishlists).unwrap_or_else(|_| "[]".to_string());
            (201, Some(response_body))
        }
        Method::Get => {
            let wishlists: Vec<Wishlist> = match store.get("wishlists")? {
                Some(value) => serde_json::from_slice(&value).unwrap_or_else(|_| Vec::new()),
                None => Vec::new(),
            };

            let response_body =
                serde_json::to_string(&wishlists).unwrap_or_else(|_| "[]".to_string());
            (200, Some(response_body))
        }
        _ => (405, None), // No other methods are currently supported
    };

    Ok(Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(body.unwrap_or_else(|| "".into()))
        .build())
}
