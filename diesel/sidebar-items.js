initSidebarItems({"fn":[["delete","Creates a delete statement. Will delete the records in the given set. Because this function has a very generic name, it is not exported by default."],["insert","Creates an insert statement. Will add the given data to a table. This function is not exported by default. As with other commands, the resulting query can return the inserted rows if you choose."],["select","Creates a bare select statement, with no from clause. Primarily used for testing diesel itself, but likely useful for third party crates as well. The given expressions must be selectable from anywhere."],["update","Creates an update statement. Helpers for updating a single row can be generated by `#[changeset_for]`."]],"macro":[["debug_sql!","Takes a query QueryFragment expression as an argument and returns a string of SQL with placeholders for the dynamic values."],["infix_predicate!","Useful for libraries adding support for new SQL types. Apps should never need to call this"],["no_arg_sql_function!","Declare a 0 argument SQL function for use in your code. This will generate a unit struct, which is an expression representing calling this function. See `now` for example output. `now` was generated using:"],["numeric_expr!","Indicates that an expression allows all numeric operators. If you create new SQL functions that return a numeric type, you should invoke this macro that type. Unfortunately, Rust disallows us from automatically implementing `Add` for types which implement `Expression`, under its orphan rules."],["operator_allowed!","Implements the Rust operator for a given type. If you create a new SQL function, which returns a type that you'd like to use an operator on, you should invoke this macro. Unfortunately, Rust disallows us from automatically implementing `Add` and other traits from `std::ops`, under its orphan rules."],["postfix_predicate!","Useful for libraries adding support for new SQL types. Apps should never need to call this."],["print_sql!","Takes takes a query QueryFragment expression as an argument and prints out the SQL with placeholders for the dynamic values."],["sql_function!","Declare a sql function for use in your code. Useful if you have your own SQL functions that you'd like to use. You can optionally provide a doc string as well. `$struct_name` should just be any unique name. You will not need to reference it in your code, but it is required due to the fact that `concat_idents!` is useless."],["table!","Specifies that a table exists, and what columns it has. This will create a new public module, with the same name, as the name of the table. In this module, you'll find a unit struct named `table`, and a unit struct with the names of each of the columns. In the definition, you can also specify an additional set of columns which exist, but should not be selected by default (for example, for things like full text search)"]],"mod":[["data_types","Structs to represent the primitive equivalent of SQL types where there is no existing Rust primitive, or where using it would be confusing (such as date and time types)"],["expression","AST types representing various typed SQL expressions. Almost all types implement either `Expression` or `AsExpression`."],["helper_types","Provide helper types for concisely writing the return type of functions. As with iterators, it is unfortunately difficult to return a partially constructed query without exposing the exact implementation of the function. Without higher kinded types, these various DSLs can't be combined into a single trait for boxing purposes."],["migrations","Provides functions for maintaining database schema."],["prelude","Re-exports important traits and types. Meant to be glob imported when using Diesel."],["query_builder","Contains traits responsible for the actual construction of SQL statements"],["query_source","Types in this module are mostly internal and automatically generated. You shouldn't need to interact with these types during normal usage, other than the methods on `Table`"],["result",""],["types","Types which represent a native SQL data type, and the conversions between them and Rust primitives. Additional types can be added by other crates."]],"struct":[["Connection",""],["Cursor","The type returned by various `Connection` methods. Acts as an iterator over `T`."]]});