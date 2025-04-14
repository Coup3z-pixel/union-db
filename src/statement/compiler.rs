use super::{node::Node, statement::Statement, types::{InvalidOperationError, OperationType, ParseTreeError, StatementCompilationError}};

pub struct StatementCompiler {
}

impl StatementCompiler {
    pub fn compile_from_str(statement_str: &str) -> Result<Statement, StatementCompilationError> {
        // tokenize
        for token in statement_str.split(" ") {
            println!("{}", token);
        }

        // convert to tree
        // operation type minus is holder value
        let tree = Self::helper(0, statement_str.split(" ").collect::<Vec<&str>>());

        match tree {
            Ok(compiled_tree) => Ok(Statement::new(compiled_tree)),
            Err(ParseTreeError) => Err(StatementCompilationError),
        }
    }

    fn helper(index: usize, tokens: Vec<&str>) -> Result<Option<Box<Node>>, ParseTreeError> {
        if let Some(token) = tokens.get(index) {

            println!("{}", token);

            if *token == "(" {
                return Self::helper(index+2, tokens);
            }

            // command
            if token.starts_with('\\') {

            }

            return Ok(Some(Box::new(Node::Set(token.to_string()))));

        } else {
            return Err(ParseTreeError);
        }
    }

    pub fn convert_symbol_to_type(symbol: &str) -> Result<OperationType, InvalidOperationError> {
        match symbol {
            "\\minus" => Ok(OperationType::Minus),
            "\\union" => Ok(OperationType::Union),
            "\\inter" => Ok(OperationType::Intersect),
            _ => Err(InvalidOperationError)
        }
    }
}
