# Project Guidelines

## General Instructions for Rust Code:

- All Rust code MUST adhere to the Rust Style Guide and rustfmt formatting.
- Use 4 spaces for indentation.
- Variable and function names should be snake_case.
- Test functions should be named test_ followed by the name of the function being tested (e.g., test_get_score_from_summary).
- Use an idiomatic Rust style which often involves a mix of functional and object-oriented patterns, prioritizing the use of structs, enums, and traits.
- Struct names and enum variants should be CamelCase.
- Use doc comments (///) for all public functions, structs, and enums, explaining their purpose, arguments, and return values.
- Use explicit type annotations where type inference isn't clear, and use const or static for constants.
- Prioritize passing arguments by reference (&) to avoid unnecessary data copies, adhering to Rust's borrowing rules.
- Use Result<T, E> and Option<T> for error handling and handling optional values, respectively.
- For testing, use Rust's built-in test module and cargo test.
- For TDD, follow the red-green-refactor cycle.

# Code Clarity and Robustness:

- Prioritize clear and simple function signatures and logic.
- Use match expressions and `if let` to improve readability where appropriate, especially when handling Result and Option types.
- Ensure robust error handling, making sure all possible error cases are handled gracefully using Result and the ? operator.
- Maintain a clean code style by removing unused imports, which the compiler will often warn about.

# Educational Context:

- When generating or explaining code, prioritize clarity and simplicity, suitable for beginners learning Rust.
- Explain concepts thoroughly, as if teaching a student.
- Avoid overly complex one-liners or macros unless specifically requested.
- Focus on core Rust principles like ownership, borrowing, and lifetimes before introducing advanced crates.

# Forbidden Actions:

- DO NOT suggest or use unsafe Rust unless explicitly asked for in a secure context demonstration and with a clear explanation of its implications.
- DO NOT recommend external crates without first discussing their purpose and alternatives with the user.
- DO NOT generate code that requires network access without explicit user permission.
- DO NOT modify files without explicit confirmation from the user.

# Preferred Tools & Workflows:

- When providing solutions, favor using Rust's standard library where possible.
- If suggesting a fix for a bug, first explain the root cause clearly, often related to ownership or borrowing.
- For file operations, always confirm with the user before reading or writing.
- Important: Prefer suggesting changes, and I will make the updates manually.  Do not prefer to update the code by yourself or automatically.
