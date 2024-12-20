//! Demonstration of an mdBook preprocessor that adds numbers to section headings.

use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use pulldown_cmark::Tag::Heading;
use pulldown_cmark::{CowStr, Event, Parser};
use std::io;

const DEEPEST_NUMBERED_SECTIONS: usize = 3; // Supports numbering up to 1.1.1

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("supports") => return, // Supports all renderers.
        Some(arg) => {
            eprintln!("Unknown argument: {arg}");
            std::process::exit(1);
        }
        None => {}
    }
    if let Err(e) = handle_preprocessing() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

struct SecNums;

impl Preprocessor for SecNums {
    fn name(&self) -> &str {
        "section-numbers"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(ch) = item {
                if !ch.is_draft_chapter() {
                    if let Err(e) = add_section_numbers(ch) {
                        eprintln!("Failed to process chapter '{}': {e:?}", ch.name);
                    }
                }
            }
        });
        Ok(book)
    }
}

fn add_section_numbers(chapter: &mut Chapter) -> Result<(), Error> {
    if let Some(ch_nums) = &chapter.number {
        if ch_nums.0.is_empty() {
            return Ok(()); // Skip empty numbering
        }

        let mut nums = ch_nums.0.clone(); // Auto-generated numbers from mdBook's TOC
        if let Some(last) = nums.last_mut() {
            *last -= 1; // Start from the previous section
        }
        nums.extend([0; 8]);

        let parser = Parser::new(&chapter.content);
        let mut events = vec![];
        for event in parser {
            if let Event::Start(Heading { level, .. }) = event {
                events.push(event.clone()); // Push the original event first
                if (level as usize) <= DEEPEST_NUMBERED_SECTIONS {
                    update_section_numbers(&mut nums, level as usize);
                    let numbering_str = format_section_numbers(&nums, level as usize);
                    events.push(Event::Text(CowStr::from(format!("{} ", numbering_str))));
                }
            } else {
                events.push(event);
            }
        }

        let mut buf = String::with_capacity(chapter.content.len() + chapter.content.len() / 8);
        chapter.content =
            pulldown_cmark_to_cmark::cmark(events.into_iter(), &mut buf).map(|_| buf)?;
    }
    Ok(())
}

fn update_section_numbers(nums: &mut Vec<u32>, level: usize) {
    nums[level - 1] += 1;
    nums[level..].fill(0); // Reset levels below the current one
}

fn format_section_numbers(nums: &[u32], level: usize) -> String {
    if level == 1 {
        if nums[0] == 1 {
            return "".to_string(); // No addition at all for the first chapter heading
        } else {
            return format!("Chapter {}: ", nums[0]); // Add "Chapter X: " for level 1
        }
    }
    nums.iter()
        .take(level)
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

fn handle_preprocessing() -> Result<(), Error> {
    let preprocessor = SecNums;
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;
    let processed_book = preprocessor.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;
    Ok(())
}
