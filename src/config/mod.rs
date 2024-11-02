pub mod language;

#[derive(Debug, PartialEq)]
pub enum PluginType {
    Input,
    Filter,
    Output,
}

#[derive(Debug, PartialEq)]
pub struct Plugin {
    name: String,
    attributes: Vec<(String, AttributeValue)>,
}

#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub enum AttributeValue {
    String(String),
    Regexp(String),
    Number(String),
}

#[derive(Debug, PartialEq)]
pub struct PluginSection {
    plugin_type: PluginType,
    plugins: Vec<Plugin>,
}
