# Rule 110 Elementary Cellular Automaton
### Author: Cristian Ion

* This program does Rule 110 Cellular Automation and prints the results.
* It starts with the 8 bit code 0b10100100.
* Following Rule 110, it computes 10 total rows.(including the first)
* When printed, 1's are replaced with *'s and 0's are replaced with .'s .
* first_row() just replaces the bits with *'s and .'s and prints the first row.
* convert() computes and returns the next row's bit from the three from the previous, following rule 110
* rule_110() converts bits to string, calls convert() to build up next row's bits, replaces and prints bits with *'s and .'s, and then returns new rows bits.
* main() calls first_row() once and then rule_110() 9 times to get the 10 rows. 
* program can be built using cargo build
* program can be ran using cargo run
* program is cargo clippy warning free
* program has been ran with cargo fmt 
* program can be tested using cargo test