use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() < 2 {
		println!("No arguments provided");
		return;
	}

	// Ensure a second parameter exists after the flag.
	if args.len() < 3 {
		println!("Missing parameter for flag {}", args[1]);
		return;
	}

	match args[1].as_str() {
		"io" => {
			println!("IO function called with {}", args[2]);
		}
		"api" => {
			println!("API function called with {}", args[2]);
		}
		"download-youtube" => {
			println!("Download YouTube function called with {}", args[2]);
		}
		"download-bluesky" => {
			println!("Download Bluesky function called with {}", args[2]);
		}
		"database" => {
			println!("Database function called with {}", args[2]);
		}
		"audio-response" => {
			println!("Audio Response function called with {}", args[2]);
		}
		"text-response" => {
			println!("Text Response function called with {}", args[2]);
		}
		"image-response" => {
			println!("Image Response function called with {}", args[2]);
		}
		"passwords" => {
			println!("Password function called with {}", args[2]);
		},
		"midday-ai" => {
			println!("Midday AI function called with {}", args[2]);
		},
		"wayback-archiving" => {
			println!("Midday AI function called with {}", args[2]);
		},
		_ => {
			println!("Unknown argument with {}", args[2]);
		}
	}
}
