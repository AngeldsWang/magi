use crate::magi_spec::*;
use std::collections::HashMap;
use std::fs;

pub fn compile_from_dsl_file(path: &str) -> Result<MagiSpec, anyhow::Error> {
    let _dsl = fs::read_to_string(path)?;
    compile_stub(&_dsl)
}

pub fn compile_stub(_source: &str) -> Result<MagiSpec, anyhow::Error> {
    // TODO: integrate real DSL parser

    Ok(MagiSpec {
        agents: vec![AgentSpec {
            name: "summarizer".into(),
            uses: "std.llm.summarize".into(),
            input: HashMap::from([
                (
                    "doc".into(),
                    Value::Interpolated("${env.DATA_DIR}/article.txt".into()),
                ),
                ("style".into(), Value::String("concise".into())),
            ]),
            depends_on: vec![],
            context: Some(ContextSpec {
                sources: vec!["memory".into(), "docs".into()],
                chain: vec![
                    ContextStep {
                        name: "std.context.filter".into(),
                        args: HashMap::from([("language".into(), "en".into())]),
                    },
                    ContextStep {
                        name: "std.context.rerank".into(),
                        args: HashMap::from([("metric".into(), "relevance".into())]),
                    },
                ],
            }),
            failure_policy: Some(FailurePolicy {
                retry_expr: Some("${max(3, env.RETRIES)}".into()),
            }),
        }],
    })
}
