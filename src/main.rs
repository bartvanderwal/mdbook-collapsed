use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::book::Book;
use mdbook::errors::Error;
use std::io::{self, Read};

// Import the CollapsedPreprocessor from lib.rs
use mdbook_collapsed::CollapsedPreprocessor;

fn main() -> Result<(), Error> {
    println!("Running mdbook-collapsed preprocessor");

    // Read the PreprocessorContext and Book from stdin (standard input)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Parse the JSON input to PreprocessorContext and Book structures
    let (ctx, book): (PreprocessorContext, Book) = serde_json::from_str(&input).unwrap();

    // Create an instance of the preprocessor from lib.rs
    let preprocessor = CollapsedPreprocessor;

    // Run the preprocessor logic from lib.rs on the book
    let processed_book = preprocessor.run(&ctx, book)?;

    // Write the modified book back to stdout (standard output) as JSON
    let output = serde_json::to_string(&processed_book).unwrap();
    println!("{}", output);

    Ok(())
}
