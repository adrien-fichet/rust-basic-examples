use serde_json::json;

fn main() -> Result<(), ureq::Error> {
    let request = ureq::get("https://httpbin.org/get?foo=bar&baz=qux")
        .header("Content-Type", "application/json");

    let response: serde_json::Value = request.call()?.body_mut().read_json()?;
    println!("{:#?}", &response);

    assert_eq!(
        &response.get("headers").unwrap().get("Host"),
        &Some(&json!("httpbin.org"))
    );

    todo!("Request containing cookies + Bearer token over HTTPS");
    //Ok(())
}
