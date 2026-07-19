# 🎯 Number Guessing Game

A simple command-line Number Guessing Game built with Rust. The program generates a random number between 1 and 100. The player keeps guessing until the correct number is found, while receiving hints after each guess.

## ✨ Features

- Generate a random number between 1 and 100
- Read user input from the terminal
- Validate user input
- Display hints for higher or lower guesses
- Continue until the correct number is guessed
- Beginner-friendly Rust implementation

## 🛠️ Technologies Used

- Rust
- Cargo
- rand crate

## 🚀 Getting Started

### Clone the Repository

```bash
git clone https://github.com/sriman1957/Rust-Projects.git
```

### Navigate to the Project

```bash
cd Rust-Projects
```

### Run the Project

```bash
cargo run
```

## 📷 Example Output

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

## 📁 Project Structure

```text
number_guessing_game/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
├── .gitignore
└── README.md
```

## 📚 Concepts Practiced

- Variables and mutability
- Primitive data types
- Loops
- User input
- String parsing
- Pattern matching with `match`
- Error handling using `Result`
- Random number generation using the `rand` crate
- Comparing values using `Ordering`

## 🔮 Future Improvements

- Add difficulty levels
- Limit the number of attempts
- Keep track of the number of guesses
- Allow players to restart the game without exiting
- Display the best score
- Add colored terminal output

## 👨‍💻 Author

**Srimannarayana Kodamasimham**

GitHub: https://github.com/sriman1957