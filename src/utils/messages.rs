use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Mutex;

lazy_static::lazy_static! {
	static ref MESSAGES: Mutex<Option<Messages>> = Mutex::new(None);
}

pub struct Messages {
	messages: HashMap<String, String>,
}

impl Messages {
	pub fn new() -> Self {
		let mut messages = HashMap::new();
		let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/utils/messages.properties");
		if let Ok(file) = File::open(&filename) {
			let reader = BufReader::new(file);
			for line in reader.lines().flatten() {
				let line = line.trim();
				if line.is_empty() || line.starts_with('#') {
					continue;
				}
				let mut parts = line.splitn(2, '=');
				if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
					messages.insert(key.to_string(), Self::parse_unicode_escapes(value));
				}
			}
		}
		Messages { messages }
	}

	/// Parse Unicode escape sequences (e.g., \u00A0) in property values
	fn parse_unicode_escapes(input: &str) -> String {
		let mut result = String::new();
		let mut chars = input.chars().peekable();
		while let Some(c) = chars.next() {
			if c == '\\' {
				if let Some('u') = chars.peek() {
					chars.next(); // consume 'u'
					let mut hex = String::new();
					for _ in 0..4 {
						if let Some(h) = chars.next() {
							hex.push(h);
						}
					}
					if let Ok(code) = u16::from_str_radix(&hex, 16) {
						if let Some(ch) = std::char::from_u32(code as u32) {
							result.push(ch);
							continue;
						}
					}
				}
			}
			result.push(c);
		}
		result
	}

	pub fn get_string(&self, key: &str) -> String {
		self.messages.get(key).cloned().unwrap_or_else(|| format!("!{}!", key))
	}
}

pub fn get_string(key: &str) -> String {
	let mut messages_guard = MESSAGES.lock().unwrap();
	if messages_guard.is_none() {
		*messages_guard = Some(Messages::new());
	}
	messages_guard.as_ref().unwrap().get_string(key)
}
