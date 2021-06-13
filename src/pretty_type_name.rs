use std::any::Any;
use std::any::type_name;
use regex::*;

pub fn pretty_type_name<T : Any>() -> String {

	let namespace_pattern = Regex::new("[_a-zA-Z0-9]+::").unwrap();

	let original_type_name = type_name::<T>();

	namespace_pattern.replace_all(original_type_name, "").to_string()

}