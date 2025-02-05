use std::env;
mod io_methods;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("No arguments provided");
		return;
	}

	// Ensure a second parameter exists after the flag.
	if args.len() < 3 {
		println!("[{}] Executing function with argument: ", args[1]);
		return;
	}

	match args[1].as_str() {
		"io" => {
			println!("[io] Executing function with argument: {}", args[2]);
			let _result = io_methods::execute_from_json(&args[2].clone());
			println!("[io] Command executed with success, result: {}", "{}");
		}
		"api" => {
			println!("[api] Executing function with argument: {}", args[2]);
			println!("[api] Command executed with success, result: {}", "{}");
		}
		"download-youtube" => {
			println!("[download-youtube] Executing function with argument: {}", args[2]);
			println!("[download-youtube] Command executed with success, result: {}", "{}");
		}
		"download-bluesky" => {
			println!("[download-bluesky] Executing function with argument: {}", args[2]);
			println!("[download-bluesky] Command executed with success, result: {}", "{}");
		}
		"database" => {
			println!("[database] Executing function with argument: {}", args[2]);
			println!("[database] Command executed with success, result: {}", "{}");
		}
		"audio-response" => {
			println!("[audio-response] Executing function with argument: {}", args[2]);
			println!("[audio-response] Command executed with success, result: {}", "{}");
		}
		"text-response" => {
			println!("[text-response] Executing function with argument: {}", args[2]);
			println!("[text-response] Command executed with success, result: {}", "{}");
		}
		"image-response" => {
			println!("[image-response] Executing function with argument: {}", args[2]);
			println!("[image-response] Command executed with success, result: {}", "{}");
		}
		"passwords" => {
			println!("[passwords] Executing function with argument: {}", args[2]);
			println!("[passwords] Command executed with success, result: {}", "{}");
		},
		"midday-ai" => {
			println!("[midday-ai] Executing function with argument: {}", args[2]);
			println!("[midday-ai] Command executed with success, result: {}", "{}");
		},
		"wayback-archiving" => {
			println!("[wayback-archiving] Executing function with argument: {}", args[2]);
			println!("[wayback-archiving] Command executed with success, result: {}", "{}");
		},
		_ => {
			println!("Unknown argument with {}", args[2]);
		}
	}
}
