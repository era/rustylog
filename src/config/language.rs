use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

use super::{AttributeValue, Plugin, PluginSection, PluginType};

#[derive(Parser)]
#[grammar = "pest/logstash_config.pest"]
struct LogStashConfigParser;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ConfigParseError {
    #[error("PEST parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),

    #[error("Unexpected parsing structure encountered")]
    UnexpectedStructure,
}

fn parse_logstash_config(input: &str) -> Result<Vec<PluginSection>, ConfigParseError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let pairs = LogStashConfigParser::parse(Rule::config, input)?; // Entry rule is `config`
    let mut sections = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::plugin_section => {
                sections.push(parse_plugin_section(pair.into_inner())?);
            }
            _ => unreachable!(),
        }
    }
    Ok(sections)
}

fn parse_plugin_section(pairs: Pairs<Rule>) -> Result<PluginSection, ConfigParseError> {
    let mut plugin_type = PluginType::Input; // Default, to be updated
    let mut plugins = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::plugin_type => {
                plugin_type = match pair.as_str() {
                    "input" => PluginType::Input,
                    "filter" => PluginType::Filter,
                    "output" => PluginType::Output,
                    _ => PluginType::Input,
                };
            }
            Rule::plugin => {
                plugins.push(parse_plugin(pair.into_inner()));
            }
            _ => return Err(ConfigParseError::UnexpectedStructure),
        }
    }

    Ok(PluginSection {
        plugin_type,
        plugins,
    })
}

fn parse_plugin(pairs: Pairs<Rule>) -> Plugin {
    let mut name = String::new();
    let mut attributes = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::name => {
                name = pair.as_str().to_string();
            }
            Rule::attribute => {
                let mut inner = pair.into_inner();
                let attr_name = inner.next().unwrap().as_str().to_owned();
                let attr_value = parse_value(inner.next().unwrap().into_inner().next().unwrap());
                attributes.push((attr_name, attr_value));
            }
            _ => unreachable!(),
        }
    }
    Plugin { name, attributes }
}

fn parse_value(pair: Pair<Rule>) -> AttributeValue {
    let val = pair.as_str().to_string();
    match pair.as_rule() {
        Rule::string => {
            let val = val[1..val.len() - 1].to_string();
            AttributeValue::String(val)
        }
        Rule::bareword => AttributeValue::String(val),
        Rule::regexp => AttributeValue::Regexp(val),
        Rule::number => AttributeValue::Number(val),
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_plugin_section() {
        let input = r#"
            input {
                file {
                    path => "/var/log/syslog"
                    codec => "json"
                }
            }
        "#;

        let expected = vec![PluginSection {
            plugin_type: PluginType::Input,
            plugins: vec![Plugin {
                name: "file".to_string(),
                attributes: vec![
                    (
                        "path".to_string(),
                        AttributeValue::String("/var/log/syslog".to_string()),
                    ),
                    (
                        "codec".to_string(),
                        AttributeValue::String("json".to_string()),
                    ),
                ],
            }],
        }];

        let result = parse_logstash_config(input).expect("input should be parsed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiple_plugin_sections() {
        let input = r#"
            input {
                file {
                    path => "/var/log/syslog"
                    codec => "json"
                }
            }
            output {
                stdout {
                    codec => "rubydebug"
                }
            }
        "#;

        let expected = vec![
            PluginSection {
                plugin_type: PluginType::Input,
                plugins: vec![Plugin {
                    name: "file".to_string(),
                    attributes: vec![
                        (
                            "path".to_string(),
                            AttributeValue::String("/var/log/syslog".to_string()),
                        ),
                        (
                            "codec".to_string(),
                            AttributeValue::String("json".to_string()),
                        ),
                    ],
                }],
            },
            PluginSection {
                plugin_type: PluginType::Output,
                plugins: vec![Plugin {
                    name: "stdout".to_string(),
                    attributes: vec![(
                        "codec".to_string(),
                        AttributeValue::String("rubydebug".to_string()),
                    )],
                }],
            },
        ];

        let result = parse_logstash_config(input).expect("input should be parsed");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = parse_logstash_config(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![]);
    }
}
