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
    pub attributes: Vec<(String, AttributeValue)>,
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
    pub plugin_type: PluginType,
    pub plugins: Vec<Plugin>,
}
