# Codestyle for contributing to red
Before you consider contributing to the project, please take the time to familiarise yourself with the code style. The code style is heavily enforced to make sure that all the code is both readable and follows good conventions.

## Variables
**Do**
```rust
// Good, only explicitly define the type when needed
let numbers: Vec<i32> = Vec::new();

let name = "Tom"; // Good, no need to mutate this
let mut score = 0; // Good, score needs to mutate 

let has_pets = true; // Good, use snake_case

// Constants
const PI = 3.14; // Good, only use uppercase for constants
const TWO_PI = PI * 2; // Good, use SHOUTING_SNAKE_CASE
```

**Don't**
```rust
let mut name = "Tom"; // No need to mutate this
let mut score: i32 = 0; // Bad, no need to define type

let haspets = true; // Bad, confusing to read
let hasPets = true; // Bad, using camelCase for the names
let HasPets = true; // Bad, using TitleCase for the names

let PI = 3.14; // Bad, constants should explicitly be const
```

## Structs
**Do**
```rust
struct Person {
    name: &str, // Good, prefer immutable &str
    age: i32 // Good, prefer i32 over other sizes
}
```

**Don't**
```rust
struct Person {
    name: String, // Bad, No need to mutate this
    age: i16 // Bad, Prefer i32 over other sizes
}
```