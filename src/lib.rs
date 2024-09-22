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

fn inject_collapsed(content: &str) -> String {
    let mut output = String::new();
    let mut in_collapsed = false;

    // Define a regex for headers that end with `#collapsed`
    let header_regex = Regex::new(r"^#+ .* #collapsed$").unwrap();

    for line in content.lines() {
        // Check if the line matches the collapsible header pattern
        if header_regex.is_match(line.trim()) {
            output.push_str(r#"<div class="collapsed-section"><h2 id="collapsed-title" onclick="toggleCollapsed()">&#x25BC; Collapsed</h2><div id="collapsed-content" style="display:none;">"#);
            in_collapsed = true;
        } else if in_collapsed && line.starts_with("#") {
            output.push_str("</div></div>"); // close collapsed div
            in_collapsed = false;
        }
        if in_collapsed {
            output.push_str(&format!(
                r#"<input type="checkbox" class="collapsed-checkbox" onchange="saveProgress()"> {line}<br>"#
            ));
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    if in_collapsed {
        output.push_str("</div></div>"); // close div's when we reach end
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
