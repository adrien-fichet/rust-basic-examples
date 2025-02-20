use serde_json::json;
use std::env;
use std::error::Error;
use std::time::Duration;
use ureq::Agent;
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    httpbin_api()?;
    cat_api()?;

    todo!("Requests reusing cookies");
    //Ok(())
}

fn httpbin_api() -> Result<(), ureq::Error> {
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
    assert_eq!(&response.get("args").unwrap().get("foo"), &Some(&json!("bar")));
    Ok(())
}

fn cat_api() -> Result<(), Box<dyn Error>> {
    const BASE_URL: &str = "https://api.thecatapi.com/v1/";
    let base_url = Url::parse(BASE_URL)?;
    let cat_api_token = env::var("CAT_API_TOKEN").expect("CAT_API_TOKEN not set"); // https://thecatapi.com/signup

    let agent: Agent = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build()
        .into();

    let request = agent
        .get(base_url.join("images/search")?.as_str())
        .header("x-api-key", &cat_api_token)
        .header("Content-Type", "application/json");

    let response: serde_json::Value = request.call()?.body_mut().read_json()?;
    println!("{:#?}", &response);

    Ok(())
}
