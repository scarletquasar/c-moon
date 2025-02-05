use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		match args[1].as_str() {
			"--io" => {
				println!("IO function called");
			}
			"--api" => {
				println!("API function called");
			}
			"--download-youtube" => {
				println!("Download YouTube function called");
			}
			"--download-bluesky" => {
				println!("Download Bluesky function called");
			},
			"--database" => {
				println!("Download Bluesky function called");
			},
			"--audio-response" => {
				println!("Audio Response function called");
			},
			"--text-response" => {
				println!("Text Response function called");
			},
			"--image-response" => {
				println!("Image response function called");
			},
			_ => {
				println!("Unknown argument");
			}
		}
	} else {
		println!("No arguments provided");
	}
}
