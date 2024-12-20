# mdbook-section-numbers

A preprocessor for mdbook that generates section numbers for headings.

This README follows the structure of books generated using mdbook and this preprocessor.

# Chapter 2: Introduction

For the first chapter heading, no modifications are made.  
For subsequent top-level headings (Level 1), "Chapter X:" is added.

## 2.1 Sections

Level 2 headings are numbered with two-part identifiers.

### 2.1.1 Subsections

Level 3 headings are numbered with three-part identifiers.

#### Subsubsection

Subsubsections remain unnumbered.

# Chapter 3: Explanation

Most online books created with mdbook display unnumbered section titles in the text, while the table of contents (TOC) includes numbering.  
In contrast, scientific books, academic papers, and theses traditionally use numbered sections throughout the text.

Manually adding section numbers is an option for those who prefer numbered text, but it complicates reordering chapters.  

The mdbook tool supports preprocessors, such as the example `remove-emphasis`. This preprocessor can be modified to automatically add section numbers to headings.

## 3.1 How to Use

1. Clone this repository:

   ```bash
   $ cd
   $ git clone https://github.com/stefansalewski/mdbook-section-numbers.git
   ```

2. Build the preprocessor:

   ```bash
   $ cd mdbook-section-numbers
   $ cargo build
   ```

   Ensure the preprocessor compiles without errors. You may need to adjust the `Cargo.toml` file.

3. Update your `book.toml` file in your mdbook project directory (where your book is stored). Add the following lines:

   ```toml
   [preprocessor.section-numbers]
   command = "cargo run --manifest-path=/home/yourname/mdbook-section-numbers/Cargo.toml --locked"
   ```

   **Note:** Replace `/home/yourname/` with the correct path for your environment.

4. Build and serve your book:

   ```bash
   $ mdbook build
   $ mdbook serve --open
   ```

## 3.2 Final Notes

This preprocessor is experimental and has not been extensively tested.  
For now, the author’s book still uses manually added section numbers, which results in duplicated numbering when the preprocessor is applied. This suggests the preprocessor works as intended.

Leaving the chapter title of Chapter 1 unmodified allows manual editing if desired. Adding "Chapter: " to Level 1 headings is an arbitrary design choice.  

Users who prefer a different numbering style can modify the preprocessor to fit their needs. The preprocessor uses the TOC numbering as a reference and assumes the text has a valid heading structure. For example, a TOC entry numbered 2.7 corresponds to a Level 2 heading ("## Some Text"). TOC entries without numbers remain unnumbered.

--- 


