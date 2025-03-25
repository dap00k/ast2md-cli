//! Functions to create DumtAST tree from the PEST parse tree (using the PEST's Pair API) and
//! pretty print the PEST parse tree for sentences of the "-dump-ast" option in the Swift compiler front-end.

use std::io::BufRead;

use pest::iterators::Pair;
use pest::Parser as PestParser;

use ast2mdlib::node_type::NodeType;
use ast2mdlib::swift_dump_ast::{Attribute, DumpAst, Node, Property};
//use crate::Rule;

#[derive(pest_derive::Parser)]
#[grammar = "swift_dump_ast.pest"]
pub struct SwiftDumpAstParser;

pub fn pest_parser(mut reader: Box<dyn BufRead>) {
    // Create a new String to store the input
    let mut input_string = String::new();

    // Read from stdin (standard input) into the String
    match reader.read_to_string(&mut input_string) {
        Ok(_) => {
            // Assign the new String to the input variable
            let dump_ast_str: &str = &input_string;

            // Just printing the parse tree (concrete grammar)
            match SwiftDumpAstParser::parse(Rule::ast, dump_ast_str) {
                Ok(parse_result) => {
                    for pair in parse_result {
                        print(pair, "");
                    }
                }
                Err(error) => {
                    eprintln!("SYNTAX ERROR: {}", error);
                }
            }

            // Constructing and printing our AST
            match SwiftDumpAstParser::parse(Rule::ast, dump_ast_str) {
                Ok(parse_result) => {
                    for pair in parse_result {
                        let ast = create_ast(pair);
                        // Pretty print the tree
                        println!("{:#?}", ast)
                    }
                }
                Err(error) => {
                    eprintln!("SYNTAX ERROR: {}", error);
                }
            }
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}

// TODO(AH): we need SyB here!

fn create_ast(pair: Pair<Rule>) -> DumpAst {
    match pair.as_rule() {
        Rule::ast => DumpAst {
            nodes: pair.into_inner().map(create_node).collect(),
        },
        _ => {
            unreachable!()
        }
    }
}

fn create_node(pair: Pair<Rule>) -> Node {
    match pair.as_rule() {
        Rule::node => {
            let mut original = false;
            let mut attributes: Vec<Attribute> = Vec::new();
            let mut nodes: Vec<Node> = Vec::new();
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::processed => original = inner.as_str() == "Original init:",
                    Rule::attribute => attributes.push(create_attribute(inner)),
                    Rule::node => nodes.push(create_node(inner)),
                    _ => {
                        unreachable!()
                    }
                }
            }

            Node::new(original, attributes, nodes, NodeType::OtherType)
        }
        _ => {
            println!("UNREACHABLE: {:?}", pair.as_rule());
            unreachable!()
        }
    }
}

fn create_attribute(pair: Pair<Rule>) -> Attribute {
    match pair.as_rule() {
        Rule::attribute => {
            if let Some(attribute) = pair.into_inner().next() {
                match attribute.as_rule() {
                    Rule::string => Attribute::String(attribute.as_str().to_string()),
                    Rule::property => Attribute::NonEmptyProperty(create_property(attribute)),
                    Rule::identifier => Attribute::EmptyProperty(attribute.as_str().to_string()),
                    _ => {
                        unreachable!()
                    }
                }
            } else {
                unreachable!()
            }
        }
        _ => {
            unreachable!()
        }
    }
}

fn create_property(pair: Pair<Rule>) -> Property {
    match pair.as_rule() {
        Rule::property => {
            let mut key: String = String::new();
            let mut value: String = String::new();
            for (index, inner) in pair.into_inner().enumerate() {
                if index == 0 {
                    key = inner.as_str().to_string();
                } else {
                    value = inner.as_str().to_string();
                }
            }

            Property { key, value }
        }
        _ => {
            unreachable!()
        }
    }
}

fn print(pair: Pair<Rule>, indent: &str) {
    let mut new_indent = indent.to_owned();
    let indentation = "  ";
    new_indent.push_str(indentation);
    match pair.as_rule() {
        Rule::ast => {
            println!("{}- ast:", indent);
            for node in pair.into_inner() {
                print(node, &new_indent);
            }
        }
        Rule::node => {
            for (index, pair) in pair.into_inner().enumerate() {
                if index == 0 {
                    println!("{}- node: {}", indent, pair.as_str());
                } else {
                    print(pair, &new_indent);
                }
            }
        }
        Rule::attribute => {
            if let Some(attribute) = pair.into_inner().next() {
                match attribute.as_rule() {
                    Rule::string => {
                        // println!("{}- attribute: {}", indent, attribute.as_str());
                        println!("{}- {}", indent, attribute.as_str());
                    }
                    Rule::property => {
                        for (index, pair) in attribute.into_inner().enumerate() {
                            if index == 0 {
                                // println!("{}- attribute: {}", indent, pair.as_str());
                                print!("{}- {}: ", indent, pair.as_str());
                            } else {
                                // println!("{}- value: {}", new_indent, pair.as_str());
                                println!("{}", pair.as_str());
                            }
                        }
                    }
                    Rule::identifier => {
                        // println!("{}- attribute: {}", indent, attribute.as_str());
                        println!("{}- {}", indent, attribute.as_str());
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
        }
        _ => {
            println!("{}- UNKNOWN RULE: {:?}", indent, pair.as_rule());
            for inner_pair in pair.into_inner() {
                print(inner_pair, &new_indent);
            }
        }
    }
}
