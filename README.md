# UniversalMask

A Rust library that makes it easy to format sensitive variables with specific masks, such as SSN, phone numbers, or other data that requires a standard format.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
universal_mask = "0.1.0"
```

## Usage

```rust
use universal_mask::mask;

// SSN formatting
let ssn = mask("123456789", "XXX-XX-XXXX");
assert_eq!(ssn, "123-45-6789");

// Phone number formatting
let phone = mask("1234567890", "(XXX) XXX-XXXX");
assert_eq!(phone, "(123) 456-7890");

// Multiple format options (separated by '|')
let number = mask("123456789012", "XXX-XX-XXXX | XX-XXXXXXX");
assert_eq!(number, "12-3456789");

// Complex formatting with longer patterns
let multi_format = mask("123456789012345", "XXX-XX-XXXX | XX-XXXXXXX | XXX-XXX-XXX-XXX-XXX");
assert_eq!(multi_format, "123-456-789-012-345");
```

## Features

- Apply masks to text according to specified formats
- Support for multiple format patterns
- Automatic selection of the most suitable format based on input length
- Minimal dependencies and efficient implementation

## Version History

### 0.1.0 - Initial Release
- Basic masking functionality
- Support for multiple format patterns
- Performance optimizations using byte operations
