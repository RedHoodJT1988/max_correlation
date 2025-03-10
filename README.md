# Maximum Correlation Coding Challenge
This was a coding challenge that was given to me to complete during an interview.
I believe that I completed it during the interview but wanted to touch on it some more
to give it a better structure and also create test cases for it.

# Steps to Run
Below are the steps in which one can take to run the program and the test cases

## Running the Program
The program itself is a CLI application that takes two inputs and then determines the maximum
correlation of the lists given. 

1. Clone this repository 
2. Change into the cloned repository locally on your machine
3. Type `cargo run` and press return/enter
4. The program will ask for input, you will need to give the program two comma separated lists
of integers.

The application should display a result of the maximum correlation similar to the following:

```bash
$ cargo run -q 
Enter numbers for array A (comma-separated):
33, , 1, 4
Enter numbers for array B (comma-spearated):
2, 5, 6
Maximum Array Correlation: 11
```

## Running the Test Cases
The program also allows you to run the test cases using `cargo test`.

1. If the repo isn't cloned already clone it
2. Change to the directory
3. Type `cargo test -q`

You will see the following output:
```bash
running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 1 test
.
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

NOTE: Using the -q command quiets the output from cargo. You can ommit that flag if you would like
to see the full output.