use crate::regex::Regex;
use lazy_static;
use std::collections::BTreeMap;

type RegexMap = BTreeMap<&'static str, Regex>;

pub mod json;
pub mod token_type;
pub mod tree;
pub mod types;
