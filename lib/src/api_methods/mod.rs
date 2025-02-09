use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Response};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Deserialize)]
struct NodeRequest {
	method: Option<String>,
	url: String,
	headers: Option<HashMap<String, String>>,
	body: Option<serde_json::Value>,
	query: Option<HashMap<String, String>>,
	form: Option<HashMap<String, String>>,
}

pub async fn execute_from_json(json_str: &str) -> Result<Response, reqwest::Error> {
	let req: NodeRequest = serde_json::from_str(&json_str).expect("Invalid JSON format");

	let method = req.method
		.as_ref()
		.map(|m| Method::from_bytes(m.as_bytes()).unwrap())
		.unwrap_or(Method::GET);

	let headers = req.headers.as_ref().map(|h| {
		h.iter().map(|(k,v)| (k.as_str(), v.as_str())).collect()
	});

	let query = req.query.as_ref().map(|q| {
		q.iter().map(|(k,v)| (k.as_str(), v.as_str())).collect()
	});

	let form = req.form.as_ref().map(|f| {
		f.iter().map(|(k,v)| (k.as_str(), v.as_str())).collect()
	});

	call_api(
		method,
		&req.url,
		query,
		req.body.as_ref(),
		form,
		headers
	).await
}

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

#[cfg(test)]
mod tests {
	use super::*;
	use reqwest::StatusCode;

	#[tokio::test]
	async fn test_execute_from_json_get() {
		let json_str = r#"
		{
			"method": "GET",
			"url": "https://httpbin.org/get",
			"headers": {
				"Accept": "application/json"
			},
			"query": {
				"key": "value"
			}
		}"#;

		let response = execute_from_json(json_str).await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_execute_from_json_post() {
		let json_str = r#"
		{
			"method": "POST",
			"url": "https://httpbin.org/post",
			"headers": {
				"Content-Type": "application/json"
			},
			"body": {
				"key": "value"
			}
		}"#;

		let response = execute_from_json(json_str).await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_execute_from_json_with_form() {
		let json_str = r#"
		{
			"method": "POST",
			"url": "https://httpbin.org/post",
			"headers": {
				"Content-Type": "application/x-www-form-urlencoded"
			},
			"form": {
				"key": "value"
			}
		}"#;

		let response = execute_from_json(json_str).await.unwrap();
		assert_eq!(response.status(), StatusCode::OK);
	}
}
