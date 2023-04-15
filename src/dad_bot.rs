use regex::Regex;
use regex::Captures;

pub fn dad_bot(message:&str,name:&str) -> String
{
	let markdown_removed:String = Regex::new(r"\*|_|\~|`|\|").unwrap().replace_all(message,"").to_string();
	let input:String = Regex::new(r"<(@!|@|@&)\d{10,25}>|@everyone|@here|(https?://[^\s]+.)").unwrap()
		.replace_all(&markdown_removed, "[REDACTED]").to_string();
	let captures: Captures = Regex::new(r"(?i)\b(i'?m|i am|i will be|i'?ve) ([^,\.\n]*)").unwrap().captures(&input).unwrap();
	let splitter: &str = captures.get(1).unwrap().as_str();
	let response: &str = captures.get(2).unwrap().as_str();
	
	return format!("hi {}, {} {}",response,splitter,name);
}