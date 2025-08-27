# 🧱 Trait-Driven Rust Architecture

This project demonstrates clean architecture principles in Rust using traits, structured error handling, and domain modeling. It's designed to showcase professional Rust development patterns and best practices.
## ✨ Key Features

- **Trait-first Architecture**: Contracts separated from implementation details
- **Feature-gated Serialization**: Choose your serialization format (JSON or Bincode)
- **Robust Error Handling**: Clear error management using `thiserror`
- **Domain Modeling**: Business models with state enums
- **Clean Layering**: Well-separated application layers

## 🛠️ Building and Running

```bash
# Run tests with bincode serialization
cargo test --features bincode

# Run tests with JSON serialization  
cargo test --features json

# Run CLI application with bincode
cargo run --bin cli --features "cli bincode"

# Run CLI application with JSON
cargo run --bin cli --features "cli json"

# Check syntax and compilation
cargo check --features bincode
```

## 📂 Project Structure

```
traity-app/
├── src/
│   ├── lib.rs              # Main library entry point
│   ├── prelude.rs          # Common imports and re-exports
│   ├── error.rs            # ErrorCode definitions and Result types
│   ├── traits/             # Trait definitions
│   │   ├── serialize.rs    # MySerialize/MyDeserialize traits
│   │   └── owner.rs        # Owner trait for access control
│   ├── domain/             # Domain models and business logic
│   │   ├── user.rs         # User struct and methods
│   │   ├── vault.rs        # Vault struct with business rules
│   │   └── state.rs        # VaultState enum for state management
│   └── infra/              # Infrastructure layer
│       └── storage.rs      # Storage trait and implementations
├── bin/
│   └── cli.rs              # Command-line interface example
└── tests/
    └── integration.rs      # Integration tests
```

## 🎯 Learning Concepts

### 1. Trait-based Design Pattern

Traits in Rust define shared behavior without specifying implementation details:

```rust
pub trait MySerialize {
    fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> Result<()>;
}

pub trait Owner {
    type OwnerId: Copy + Eq + Debug;
    fn owner_id(&self) -> Self::OwnerId;
}
```

**Why use traits?**
- **Flexibility**: Different types can implement the same behavior differently
- **Testability**: Easy to mock dependencies in tests
- **Extensibility**: Add new implementations without changing existing code

### 2. Feature-gated Implementations

Rust's feature flags allow conditional compilation:

```rust
#[cfg(feature = "bincode")]
mod bincode_impls {
    // Implementation using bincode for binary serialization
}

#[cfg(feature = "json")]  
mod json_impls {
    // Implementation using JSON for text serialization
}
```

**Benefits:**
- **Reduced dependencies**: Only compile what you need
- **Flexibility**: Choose serialization format at compile time
- **Performance**: Binary vs. human-readable trade-offs

### 3. State Management with Enums

Rust enums are powerful for modeling business states:

```rust
pub enum VaultState {
    Uninitialized,
    Active { frozen: bool },
    Closed,
}

// Usage with pattern matching
impl Vault {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        match self.state {
            VaultState::Active { frozen: false } => {
                self.balance += amount;
                Ok(())
            }
            VaultState::Active { frozen: true } => {
                Err(ErrorCode::VaultFrozen)
            }
            _ => Err(ErrorCode::InvalidState),
        }
    }
}
```

**Advantages:**
- **Type safety**: Compiler ensures all states are handled
- **Clarity**: Business rules are explicit in code
- **Maintainability**: Easy to add new states and transitions

### 4. Structured Error Handling

Using `thiserror` for clean error definitions:

```rust
#[derive(Debug, Error)]
pub enum ErrorCode {
    #[error("Invalid data format")]
    InvalidData,
    
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
    
    #[error("Not owner of this resource")]
    NotOwner,
    
    #[error("Vault is currently frozen")]
    VaultFrozen,
    
    #[error("Invalid state for this operation")]
    InvalidState,
}
```

**Best practices:**
- **Descriptive messages**: Help users understand what went wrong
- **Categorization**: Group related errors for better handling
- **Context**: Include relevant information in error variants

## 🚀 Getting Started

### Step 1: Clone and Build
```bash
git clone <your-repo>
cd traity-app
cargo build --features bincode
```

### Step 2: Run Tests
```bash
# Test core functionality
cargo test --features bincode -- --nocapture

# Test with different serialization
cargo test --features json -- --nocapture
```

### Step 3: Try the CLI
```bash
# Create a new vault and perform operations
cargo run --bin cli --features "cli bincode"
```

## 🔧 Troubleshooting

### Build Issues

If you encounter the error `unknown proxy name: 'Cursor-1.4.5-x86_64'`:

```bash
# Update Rust toolchain
rustup self update
rustup update stable
rustup default stable

# Or reinstall Rust completely
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Common Problems

**Problem**: Features not working as expected
**Solution**: Make sure you're using the correct feature flags:
```bash
# Wrong - missing features
cargo run --bin cli

# Correct - with required features
cargo run --bin cli --features "cli bincode"
```

**Problem**: Serialization errors
**Solution**: Check that your data structures implement the required traits for your chosen serialization format.

## 📚 Learning Resources

### Essential Reading
- [The Rust Book](https://doc.rust-lang.org/book/) - Start here for Rust fundamentals
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) - Hands-on examples
- [Effective Rust](https://www.lurklurk.org/effective-rust/) - Best practices and idioms

### Advanced Topics
- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/) - Common patterns in Rust
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Advanced unsafe Rust concepts
- [Async Rust](https://rust-lang.github.io/async-book/) - Asynchronous programming

