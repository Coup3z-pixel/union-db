use super::statement::Statement;

pub struct StatementExecutor {
}

impl StatementExecutor {
    pub fn new() -> Self {
        StatementExecutor{}
    }

    pub fn execute(&self, statement: Statement) {
        statement.print_tree();
    }
}
