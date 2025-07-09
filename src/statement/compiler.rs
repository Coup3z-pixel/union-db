use super::{
    node::Node, 
    statement::Statement, 
    types::{
        self, 
        InvalidOperationError, 
        ParseTreeError, 
        StatementCompilationError
    }
};

pub struct StatementCompiler {
}

impl StatementCompiler {
    pub fn compile_from_str(statement_str: &str) -> Result<Statement, StatementCompilationError> {
        if !Self::is_valid_statement(statement_str) {
            return Err(StatementCompilationError);
        }

        // Tokenize the input string
        let tokens: Vec<&str> = statement_str.split_whitespace().collect();
        
        // Parse the expression into a tree
        let tree = Self::parse_expression(&tokens);

        match tree {
            Ok(compiled_tree) => Ok(Statement::new(compiled_tree)),
            Err(ParseTreeError) => {
                println!("Failed to parse expression: {}", statement_str);
                Err(StatementCompilationError)
            },
        }
    }

    fn parse_expression(tokens: &[&str]) -> Result<Option<Box<Node>>, ParseTreeError> {
        if tokens.is_empty() {
            return Ok(None);
        }

        if tokens.len() == 2 {
            match Self::parse_term(tokens, 0) {
                Ok((design_schema, _)) => return Ok(design_schema),
                Err(ParseTreeError) => return Err(ParseTreeError)
            };
        }

        // Parse the expression using a recursive descent parser
        let (node, _) = Self::parse_expression_recursive(tokens, 0)?;
        Ok(node)
    }

    fn parse_expression_recursive(tokens: &[&str], pos: usize) -> Result<(Option<Box<Node>>, usize), ParseTreeError> {
        if pos >= tokens.len() {
            return Ok((None, pos));
        }

        // Parse left operand
        let (mut left, mut current_pos) = Self::parse_term(tokens, pos)?;
        
        // Continue parsing operations as long as we find them
        while current_pos < tokens.len() {
            let token = tokens[current_pos];
            
            // Check if we've reached the end of this expression
            if token == ")" {
                return Ok((left, current_pos + 1));
            }
            
            // Check if we have an operation
            if token.starts_with('\\') {
                // Get operation type
                let op_type = match types::convert_symbol_to_type(token) {
                    Ok(operation_type) => operation_type,
                    Err(InvalidOperationError) => {
                        println!("'{}' is not a valid operation", token);
                        return Err(ParseTreeError);
                    },
                };
                
                // Parse right operand
                let (right, new_pos) = Self::parse_term(tokens, current_pos + 1)?;
                
                // Create operation node
                left = Some(Box::new(Node::Operation(
                    op_type,
                    left,
                    right
                )));
                
                current_pos = new_pos;
            } else {
                // If not an operation and not a closing parenthesis, something is wrong
                if token != ")" {
                    println!("Unexpected token: {}", token);
                    return Err(ParseTreeError);
                }
                break;
            }
        }
        
        Ok((left, current_pos))
    }
    
    fn parse_term(tokens: &[&str], pos: usize) -> Result<(Option<Box<Node>>, usize), ParseTreeError> {
        if pos >= tokens.len() {
            return Ok((None, pos));
        }
        
        let token = tokens[pos];
        
        // Handle parenthesized expressions
        if token == "(" {
            let (node, new_pos) = Self::parse_expression_recursive(tokens, pos + 1)?;
            return Ok((node, new_pos));
        }
        
        // Handle set identifiers
        if !token.starts_with('\\') && token != "(" && token != ")" {
            return Ok((Some(Box::new(Node::Set(token.to_string()))), pos + 1));
        }

        // database design
        if token.starts_with('\\') {

            let op_type = match types::convert_symbol_to_type(token) {
                Ok(operation_type) => operation_type,
                Err(InvalidOperationError) => {
                    println!("'{}' is not a valid operation", token);
                    return Err(ParseTreeError);
                },
            };


            let new_set_node = Node::Set(tokens[pos].to_string());
            let design_node = Node::Operation(op_type, None, Some(Box::new(new_set_node)));
            return Ok((Some(Box::new(design_node)), pos));
        }
        
        // If we get here, we have an unexpected token
        println!("Unexpected token in term: {}", token);
        Err(ParseTreeError)
    } 

    fn is_valid_statement(statement_str: &str) -> bool {
        return Self::valid_matching_bracket(statement_str);
    }

    fn valid_matching_bracket(expression_str: &str) -> bool {
        let mut open: i8 = 0;
        let mut close: i8 = 0;

        for char in expression_str.chars() {
            if char == '(' {
                open = open + 1;
            } else if char == ')' {
                close = close + 1;
            }

            if close > open {
                return false;
            }
        }

        return true;
    }
}
