use std::io::Read;
use std::io::Write;
use serde::Deserialize;

pub fn execute_from_json(json_str: &str) -> String {
	#[derive(Deserialize)]
	struct Command {
		method: String,
		#[serde(rename = "type")]
		typ: String,
		content: Option<String>,
		name: Option<String>,
	}

	let command: Command =
		serde_json::from_str(&json_str).expect("[io] JSON format is invalid");
	
	match (command.method.as_str(), command.typ.as_str()) {
		("read", "file") => {
			let file_name = command
				.name
				.as_deref()
				.expect("[io] File name is required for reading");
			match read_from_file(file_name) {
				Ok(contents) => format!("[io] [result] {}", contents),
				Err(e) => format!("[io] [error] Failed to read file: {}", e),
			}
		},
		("read", "folder") => {
			let folder_name = command
				.name
				.as_deref()
				.expect("[io] Folder name is required for listing contents");
			match std::fs::read_dir(folder_name) {
				Ok(entries) => {
					let mut res = String::new();
					for entry in entries.filter_map(Result::ok) {
						if let Ok(file_type) = entry.file_type() {
							let type_str = if file_type.is_dir() {
								"folder"
							} else if file_type.is_file() {
								"file"
							} else {
								"other"
							};
							res.push_str(&format!("{}: {:?}\n", type_str, entry.file_name()));
						}
					}
					format!("[io] [result]\n{}", res)
				},
				Err(e) => format!("[io] [error] Failed to read folder: {}", e),
			}
		},
		("create", "file") => {
			let file_name = command
				.name
				.as_deref()
				.expect("[io] File name is required for creation");
			let content = command.content.unwrap_or_default();
			match create_file(file_name, &content) {
				Ok(_) => format!("[io] [result] Created file: {}", file_name),
				Err(e) => format!("[io] [error] Failed to create file: {}", e),
			}
		},
		("update", "file") => {
			let file_name = command
				.name
				.as_deref()
				.expect("[io] File name is required for update");
			let content = command.content.unwrap_or_default();
			match update_file(file_name, &content) {
				Ok(_) => format!("[io] [result] Updated file: {}", file_name),
				Err(e) => format!("[io] [error] Failed to update file: {}", e),
			}
		},
		("delete", "file") => {
			let file_name = command
				.name
				.as_deref()
				.expect("[io] File name is required for deletion");
			match delete_file(file_name) {
				Ok(_) => format!("[io] [result] Deleted file: {}", file_name),
				Err(e) => format!("[io] [error] Failed to delete file: {}", e),
			}
		},
		("create", "folder") => {
			let folder_name = command
				.name
				.as_deref()
				.expect("[io] Folder name is required for creation");
			match create_folder(folder_name) {
				Ok(_) => format!("[io] [result] Created folder: {}", folder_name),
				Err(e) => format!("[io] [error] Failed to create folder: {}", e),
			}
		},
		("update", "folder") => {
			// For folder update, assume we rename a folder named "default_folder" to the new name.
			let new_folder_name = command
				.name
				.as_deref()
				.expect("[io] New folder name is required for update");
			match update_folder("default_folder", new_folder_name) {
				Ok(_) => format!("[io] [result] Renamed folder 'default_folder' to {}", new_folder_name),
				Err(e) => format!("[io] [error] Failed to update folder: {}", e),
			}
		},
		("delete", "folder") => {
			let folder_name = command
				.name
				.as_deref()
				.expect("[io] Folder name is required for deletion");
			match delete_folder(folder_name) {
				Ok(_) => format!("[io] [result] Deleted folder: {}", folder_name),
				Err(e) => format!("[io] [error] Failed to delete folder: {}", e),
			}
		},
		_ => format!("[io] [error] Invalid command"),
	}
}

pub fn create_file(name: &str, content: &str) -> std::io::Result<()> {
	write_to_file(name, content)
}

pub fn update_file(name: &str, content: &str) -> std::io::Result<()> {
	// For update, we simply overwrite the file.
	write_to_file(name, content)
}

pub fn delete_file(name: &str) -> std::io::Result<()> {
	std::fs::remove_file(name)
}

pub fn create_folder(name: &str) -> std::io::Result<()> {
	std::fs::create_dir(name)
}

pub fn update_folder(old_name: &str, new_name: &str) -> std::io::Result<()> {
	std::fs::rename(old_name, new_name)
}

pub fn delete_folder(name: &str) -> std::io::Result<()> {
	std::fs::remove_dir_all(name)
}

pub fn read_from_file<P: AsRef<std::path::Path>>(path: P) -> std::io::Result<String> {
	let mut file = std::fs::File::open(path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}

pub fn write_to_file<P: AsRef<std::path::Path>>(path: P, contents: &str) -> std::io::Result<()> {
	let mut file = std::fs::File::create(path)?;
	file.write_all(contents.as_bytes())?;
	Ok(())
}

#[cfg(test)]
mod io_methods_tests {
	use super::*;
	use std::fs::{self};
	use std::path::Path;

	#[test]
	fn test_read_write() {
		let test_path = "test_file.txt";
		let content = "Hello, Rust!";
		write_to_file(test_path, content).expect("Failed to write file");
		let read_content = read_from_file(test_path).expect("Failed to read file");
		assert_eq!(content, read_content);
		fs::remove_file(test_path).expect("Failed to remove test file");
	}

	#[test]
	fn test_create_file() {
		let test_path = "test_create_file.txt";
		let content = "File content";
		create_file(test_path, content).expect("Failed to create file");
		let read_content = read_from_file(test_path).expect("Failed to read file");
		assert_eq!(content, read_content);
		fs::remove_file(test_path).expect("Failed to remove test file");
	}

	#[test]
	fn test_update_file() {
		let test_path = "test_update_file.txt";
		let initial_content = "Initial content";
		let updated_content = "Updated content";
		write_to_file(test_path, initial_content).expect("Failed to write initial file");
		update_file(test_path, updated_content).expect("Failed to update file");
		let read_content = read_from_file(test_path).expect("Failed to read file");
		assert_eq!(updated_content, read_content);
		fs::remove_file(test_path).expect("Failed to remove test file");
	}

	#[test]
	fn test_delete_file() {
		let test_path = "test_delete_file.txt";
		let content = "File to be deleted";
		write_to_file(test_path, content).expect("Failed to write file");
		delete_file(test_path).expect("Failed to delete file");
		assert!(!Path::new(test_path).exists());
	}

	#[test]
	fn test_create_folder() {
		let test_folder = "test_create_folder";
		create_folder(test_folder).expect("Failed to create folder");
		assert!(Path::new(test_folder).is_dir());
		fs::remove_dir(test_folder).expect("Failed to remove test folder");
	}

	#[test]
	fn test_update_folder() {
		let old_folder = "test_old_folder";
		let new_folder = "test_new_folder";
		create_folder(old_folder).expect("Failed to create old folder");
		update_folder(old_folder, new_folder).expect("Failed to rename folder");
		assert!(!Path::new(old_folder).exists());
		assert!(Path::new(new_folder).is_dir());
		fs::remove_dir(new_folder).expect("Failed to remove test folder");
	}

	#[test]
	fn test_delete_folder() {
		let test_folder = "test_delete_folder";
		create_folder(test_folder).expect("Failed to create folder");
		delete_folder(test_folder).expect("Failed to delete folder");
		assert!(!Path::new(test_folder).exists());
	}

	#[test]
	fn test_execute_from_json() {
		let create_file_json = r#"{"method": "create", "type": "file", "name": "test_json_file.txt", "content": "JSON content"}"#;
		let result = execute_from_json(create_file_json);
		assert!(result.contains("[io] [result] Created file: test_json_file.txt"));
		let read_content = read_from_file("test_json_file.txt").expect("Failed to read file");
		assert_eq!(read_content, "JSON content");
		fs::remove_file("test_json_file.txt").expect("Failed to remove test file");

		let create_folder_json = r#"{"method": "create", "type": "folder", "name": "test_json_folder"}"#;
		let result = execute_from_json(create_folder_json);
		assert!(result.contains("[io] [result] Created folder: test_json_folder"));
		assert!(Path::new("test_json_folder").is_dir());
		fs::remove_dir("test_json_folder").expect("Failed to remove test folder");
	}
}
