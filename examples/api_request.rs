use serde_json::json;

fn main() -> Result<(), ureq::Error> {
    let request = ureq::get("https://httpbin.org/get")
        .query("foo", "bar")
        .query("baz", "quz")
        .header("Content-Type", "application/json");

    let response: serde_json::Value = request.call()?.body_mut().read_json()?;
    println!("{:#?}", &response);

    assert_eq!(
        &response.get("headers").unwrap().get("Host"),
        &Some(&json!("httpbin.org"))
    );
    assert_eq!(
        &response.get("args").unwrap().get("foo"),
        &Some(&json!("bar"))
    );

    todo!("Request containing cookies + Bearer token over HTTPS");
    //Ok(())
}
