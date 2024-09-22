use mdbook::book::BookItem;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::book::Book;
use regex::Regex;

// use mdbook::renderer::RenderContext;
// use pulldown_cmark::{Event, Options, Parser, Tag};
// use std::process;

pub struct CollapsedPreprocessor;

impl Preprocessor for CollapsedPreprocessor {
    fn name(&self) -> &str {
        "collapsed-preprocessor"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        for section in &mut book.sections {
            if let BookItem::Chapter(ref mut chapter) = section {
                let new_content = inject_collapsed(&chapter.content);
                chapter.content = new_content;
            }
        }
        Ok(book)
    }
}

pub fn inject_collapsed(content: &str) -> String {
    let mut output = String::new();
    let mut in_collapsed = false;
    let mut collapse_level = 0;

    // Define a regex for headers that end with `#collapsed`
    let header_regex = Regex::new(r"^(#+) .* #collapsed$").unwrap();
    let general_header_regex = Regex::new(r"^(#+) ").unwrap(); // Match any header

    for line in content.lines() {
        // Check if the line matches the collapsible header pattern
        if let Some(caps) = header_regex.captures(line.trim()) {
            // Start a collapsible section
            let hashes = &caps[1]; // Capture the number of `#`
            collapse_level = hashes.len(); // Set the current collapse level
            output.push_str(
                r#"<div class="collapsed-section"><h2 id="collapsed-title" onclick="toggleCollapsed()">&#x25BC; Collapsed</h2><div id="collapsed-content" style="display:none;">"#,
            );
            in_collapsed = true;
        } else if in_collapsed {
            // Check if the line starts with a header
            if let Some(caps) = general_header_regex.captures(line.trim()) {
                let hashes = &caps[1];
                let header_level = hashes.len(); // Get the level of the new header
                
                // If the new header level is less than or equal to the collapse level, close the div
                if header_level <= collapse_level {
                    output.push_str("</div></div>"); // close collapsed div
                    in_collapsed = false;
                }
            }
        }

        // Append the line to the output, regardless of whether it's in a collapsed section
        output.push_str(line);
        output.push('\n');
    }

    // Ensure any open collapsible section is closed
    if in_collapsed {
        output.push_str("</div></div>"); // close the last collapsed div if it's still open
    }

    output
}

//fn main() {
    // let _preprocessor = CollapsedPreprocessor;
    // if let Err(e) = Preprocessor::run(&preprocessor, &PreprocessorContext::default(), &mut Book::new()) {
    //     eprintln!("{}", e);
    //     process::exit(1);
    // }
// }

// Standaard
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
