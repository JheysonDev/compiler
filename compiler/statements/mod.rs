mod import;
mod library;

use crate::Environment;
use crate::expressions::evaluate as evaluate_expression;
use crate::objects::*;

use sflyn_parser::statements::Statements;

pub fn evaluate(
  file_name: String,
  statement: Box<Statements>,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  // Block
  if statement.clone().is_block() {
    let mut result_object: Option<Box<Objects>> = None;

    for statement in statement.clone().get_block().unwrap().statements {
      result_object = evaluate(file_name.clone(), statement.clone(), environment);

      match result_object.clone() {
        Some(object) => {
          // Check if the result object is an error, return or print object.
          if object.clone().is_error() ||
            object.clone().is_return() ||
            object.clone().is_print() {
            return result_object;
          }
        },
        None => {},
      }
    }

    return result_object;
  }

  // Export
  if statement.clone().is_export() {
    return match statement.clone().get_export().unwrap().value {
      Some(value) => evaluate(file_name.clone(), value, environment),
      None => None,
    };
  }

  // Expression
  if statement.clone().is_expression() {
    return Some(evaluate_expression(
      file_name.clone(),
      statement.clone().get_expression().unwrap().expression,
      environment,
    ));
  }

  // Function
  if statement.clone().is_function() {
    let function = statement.clone().get_function().unwrap();

    // Add default values.
    AnonymousFunction::add_arguments_to_environment(
      file_name.clone(),
      function.arguments.clone(),
      environment,
    );

    let function_object = AnonymousFunction::new(
      true,
      function.arguments.clone(),
      function.data_type.clone(),
      function.body.clone(),
      environment.clone(),
    );

    environment.set(function.name.string(), function_object);
  }

  // Import
  if statement.clone().is_import() {
    return import::evaluate(
      file_name.clone(),
      statement.clone().get_import().unwrap(),
      environment,
    );
  }

  // Library
  if statement.clone().is_library() {
    return Some(library::evaluate(
      file_name.clone(),
      statement.clone().get_library().unwrap(),
      environment,
    ));
  }

  // Return
  if statement.clone().is_return() {
    // Compile return value.
    let value_object = evaluate_expression(
      file_name.clone(),
      statement.clone().get_return().unwrap().value,
      environment,
    );

    // Check if the value object is an error.
    if value_object.clone().is_error() {
      return Some(value_object);
    }

    // Return a new return object.
    return Some(ReturnO::new(value_object));
  }

  // Variable
  if statement.clone().is_variable() {
    // Get variable statement.
    let variable = statement.clone().get_variable().unwrap();

    // Compile variable value.
    let object = evaluate_expression(file_name.clone(), variable.value, environment);

    // Check if the object is an error.
    if object.clone().is_error() {
      return Some(object);
    }

    // Add variable value to the environment.
    environment.set(variable.name.string(), object);
  }

  // Variable set

  None
}
