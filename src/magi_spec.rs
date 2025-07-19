use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MagiSpec {
    pub agents: Vec<AgentSpec>,
}

#[derive(Debug, Clone)]
pub struct AgentSpec {
    pub name: String,
    pub uses: String,
    pub input: HashMap<String, Value>,
    pub depends_on: Vec<String>,
    pub context: Option<ContextSpec>,
    pub failure_policy: Option<FailurePolicy>,
}

#[derive(Debug, Clone)]
pub struct ContextSpec {
    pub sources: Vec<String>,
    pub chain: Vec<ContextStep>,
}

#[derive(Debug, Clone)]
pub struct ContextStep {
    pub name: String,
    pub args: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct FailurePolicy {
    pub retry_expr: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Interpolated(String),
    FunctionCall { name: String, args: Vec<Value> },
    Array(Vec<Value>),
    Object(HashMap<String, Value>),
}
