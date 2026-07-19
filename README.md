````markdown
# Number Guessing Game

A simple command-line number guessing game built with Rust. The program generates a random number between 1 and 100, accepts user input, validates the input, and provides hints until the correct number is guessed.

## Features

- Random number generation using the `rand` crate
- User input through the terminal
- Input validation with error handling
- Hints for higher or lower guesses
- Infinite game loop until the correct guess is entered
- Clean and beginner-friendly Rust implementation

## Technologies Used

- Rust
- Cargo
- rand crate

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/sriman1957/Rust-Projects.git
```

### Navigate to the Project

```bash
cd Rust-Projects/number_guessing_game
```

### Run the Project

```bash
cargo run
```

## Example Output

```text
Number Guessing Game

Please enter your guess:
50
Your guess is too small.

Please enter your guess:
75
Your guess is too big.

Please enter your guess:
63
Congratulations! You guessed the correct number.
```

## Project Structure

```text
number_guessing_game/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

## Concepts Practiced

- Variables and mutability
- Data types
- Loops
- User input
- String parsing
- Pattern matching with `match`
- Error handling using `Result`
- Random number generation
- Value comparison using `Ordering`

## Future Improvements

- Limit the number of attempts
- Add difficulty levels
- Allow replay without restarting the program
- Track the number of attempts
- Display a high score
- Add colored terminal output

## Author

**Srimannarayana Kodamasimham**
```
````
