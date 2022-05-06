# README
Date: 11/07/21

## Compilation Instructions:
To compile the parser program using Terminal (MacOS) or on any CSIF computers, use the following console command: "cargo build" (excluding the quotes)

## Run and Input Instructions:
To run the compiled code, type the following into console: "./target/debug/parser {file name path}" (excluding the quotes) where file name path is the path to the given file of interest.

## Output
### Console Output:
The console will output three sets of information. The first set is the matching of token type to each respective token. The second set of output, if necessary, will describe any EBNF syntactic errors found in the code. Finally, the console will alert the user whether or not the creation of the output 'result.xhtml' file was successful. 

### XHTML Output:
The program after encountering either an EBNF error or finishing successfully will output a xhtml file called 'result.xhtml' which can be found in the parser home directory.

## Error Handling:
Whenever poor syntax is detected, the program will exit with exit(1)

## References:
1. https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
2. https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
3. https://riptutorial.com/rust/example/4276/write-in-a-file
4. https://stackoverflow.com/questions/38304666/how-to-define-a-copyable-struct-containing-a-string
5. https://doc.rust-lang.org/rust-by-example/flow_control/match.html
6. https://stackoverflow.com/questions/25576748/how-to-compare-enum-without-pattern-matching
7. https://stackoverflow.com/questions/25569865/how-to-escape-curly-braces-in-a-format-string-in-rust


