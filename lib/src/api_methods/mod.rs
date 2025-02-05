use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

pub async fn call_api(
	method: Method,
	url: &str,
	query: Option<HashMap<&str, &str>>,
	body: Option<impl Serialize>,
	form: Option<HashMap<&str, &str>>,
	headers: Option<HashMap<&str, &str>>,
) -> Result<Response, reqwest::Error> {
	let client = Client::new();
	let mut request_builder = client.request(method, url);

	if let Some(query_params) = query {
		request_builder = request_builder.query(&query_params);
	}

	if let Some(body_content) = body {
		request_builder = request_builder.json(&body_content);
	}

	if let Some(form_data) = form {
		request_builder = request_builder.form(&form_data);
	}

	if let Some(header_map) = headers {
		let mut header_map_converted = HeaderMap::new();
		for (key, value) in header_map {
			header_map_converted.insert(HeaderName::from_str(key).unwrap(), HeaderValue::from_str(&value.to_string()).unwrap());
		}
		request_builder = request_builder.headers(header_map_converted);
	}

	let response = request_builder.send().await?;
	Ok(response)
}