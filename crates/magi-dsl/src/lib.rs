use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MagiParser;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentSpec {
    pub name: String,
    pub planner: String,
    pub executor: String,
    pub tools: Vec<String>,
    pub memory: HashMap<String, String>,
    pub personality: HashMap<String, String>,
    pub policy: HashMap<String, String>,
}

fn extract_value(pair: pest::iterators::Pair<Rule>) -> String {
    let mut inner = pair.into_inner();
    let inner_val = inner.next().unwrap();
    match inner_val.as_rule() {
        Rule::string => inner_val.as_str().trim_matches('"').to_string(),
        Rule::ident => inner_val.as_str().to_string(),
        _ => "".into(),
    }
}

pub fn parse_agent(input: &str) -> Result<AgentSpec> {
    let mut parsed =
        MagiParser::parse(Rule::agent_def, input).map_err(|e| anyhow!("Parse error: {e}"))?;
    let agent = parsed.next().unwrap();
    let mut name = String::new();
    let mut planner = String::new();
    let mut executor = String::new();
    let mut tools = Vec::new();
    let mut memory = HashMap::new();
    let mut personality = HashMap::new();
    let mut policy = HashMap::new();

    for pair in agent.into_inner() {
        match pair.as_rule() {
            Rule::string => name = pair.as_str().trim_matches('"').to_string(),
            Rule::kv_pair => {
                let mut inner = pair.into_inner();
                let key = inner.next().unwrap().as_str();
                let value = extract_value(inner.next().unwrap());
                match key {
                    "planner" => planner = value,
                    "executor" => executor = value,
                    _ => {}
                }
            }
            Rule::tools_list => {
                for t in pair.into_inner() {
                    if t.as_rule() == Rule::value {
                        tools.push(extract_value(t));
                    }
                }
            }
            Rule::memory_block => {
                for kv in pair.into_inner() {
                    let mut inner = kv.into_inner();
                    let key = inner.next().unwrap().as_str();
                    let value = extract_value(inner.next().unwrap());
                    memory.insert(key.to_string(), value);
                }
            }
            Rule::personality_block => {
                for kv in pair.into_inner() {
                    let mut inner = kv.into_inner();
                    let key = inner.next().unwrap().as_str();
                    let value = extract_value(inner.next().unwrap());
                    personality.insert(key.to_string(), value);
                }
            }
            Rule::policy_block => {
                for kv in pair.into_inner() {
                    let mut inner = kv.into_inner();
                    let key = inner.next().unwrap().as_str();
                    let value = extract_value(inner.next().unwrap());
                    policy.insert(key.to_string(), value);
                }
            }
            _ => {}
        }
    }

    Ok(AgentSpec {
        name,
        planner,
        executor,
        tools,
        memory,
        personality,
        policy,
    })
}
