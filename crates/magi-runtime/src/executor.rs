pub struct Executor {
    // task graph, retry logic, logging
}
impl Executor {
    pub fn run_plan(&self, plan: PlanSpec) -> Result<(), ExecError> {
        // topological sort + dispatch tasks
    }
}
