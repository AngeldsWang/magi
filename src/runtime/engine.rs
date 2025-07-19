use crate::magi_spec::MagiSpec;

pub fn run(spec: &MagiSpec) -> anyhow::Result<()> {
    println!("[magi::engine] Executing agent DAG...");

    for agent in &spec.agents {
        println!(
            "[magi::run] → Agent `{}` using `{}`",
            agent.name, agent.uses
        );
        println!("           Inputs: {:?}", agent.input);
        if let Some(ctx) = &agent.context {
            println!("           Context sources: {:?}", ctx.sources);
            for step in &ctx.chain {
                println!("           ⤷ {} {:?}", step.name, step.args);
            }
        }
        println!("           ✔ finished\n");
    }

    Ok(())
}
