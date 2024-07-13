# Lambda
Implementing "Types and Programming Languages" chapter 10 and 11 in Rust.

Main goal is not to just create compiler and interpreter, but also to create IDE support with tree-sitter and working LSP.

Therefore I cut the scope of this language to bare minimum (pure lambda calculus), focus on tooling from the first commit and only then add more features described in chapter 11.

# Chapter 10

* [x] If else
* [x] Type checking
* [x] Evaluating
* [x] Tree sitter grammar
* [x] Syntax highlighting
* [/] LSP
    * [x] Hover
    * [ ] Completion
    * [x] Diagnostics
    * [ ] Formatting
    * [x] Go to definition
    * [ ] Find references
    * [ ] Rename
    * [x] Inlay hints
* [ ] Formatter
* [ ] REPL
* [ ] Modules
    This is not in the book, but for the sake of making
    the LSP and other parts more extensible, I will add
    simple `import` syntax. This means, all parts of the compiler have to be 
    able to handle multiple sources of code.
* [ ] Numbers & Booleans

# Chapter 11
* [ ] Unit type
* [ ] Ascription
* [x] Let
    * [x] Let polymorphism
* [ ] Pairs
* [ ] Tuples
* [ ] Records
* [ ] Sums
* [ ] Variants
* [ ] General recursion
* [ ] Lists