use std::collections::HashMap;

pub mod language;

#[derive(Debug, PartialEq)]
pub enum PluginType {
    Input,
    Filter,
    Output,
}

#[derive(Debug, PartialEq)]
pub struct Plugin {
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum AttributeValue {
    String(String),
    Regexp(String),
    Number(String),
}

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        match self {
            AttributeValue::Number(string)
            | AttributeValue::Regexp(string)
            | AttributeValue::String(string) => string.clone(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PluginSection {
    pub plugin_type: PluginType,
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum InputConfigOption {
    Name,
}

impl ToString for InputConfigOption {
    fn to_string(&self) -> String {
        match self {
            InputConfigOption::Name => String::from("name"),
        }
    }
}
