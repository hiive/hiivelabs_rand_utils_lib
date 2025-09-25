# HiiveLabs Random Utils Library

A comprehensive Rust utility library providing random number generation, string manipulation, coordinate systems, weighted selection, and worker pool functionality for game development and procedural generation.

## Features

- **Grid Coordinate System**: Type-safe coordinate handling with `IDim`, `TIndex`, and `UDim` types
- **Seed Utilities**: Deterministic random seed generation from byte arrays
- **String Utilities**: Case conversion functions (title case, underscore case)
- **Weighted Selection**: `WeightedPicker` for probability-based item selection
- **Worker Pool**: Multi-threaded task execution system with message passing
- **Logging Support**: Integrated logging with configurable output

## Dependencies

- `blake2` - Cryptographic hashing for seed generation
- `log` - Logging framework
- `log4rs` - Advanced logging configuration
- `itertools` - Iterator utilities
- `rand` - Random number generation
- `getrandom` - Cross-platform random number generation (WebAssembly compatible)
- `enum-map` - Efficient enum-based maps
- `lazy_static` - Static initialization
- `schemars` - JSON schema generation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
hiivelabs_rand_utils_lib = { version = "0.1.0", path = "path/to/hiivelabs_rand_utils_lib" }
```

### Basic Examples

#### Coordinate System
```rust
use hiivelabs_rand_utils_lib::prelude::*;

// Work with type-safe coordinates
let coord: IDim = 10;
let index: TIndex = 5;
let dimension: UDim = 100;
```

#### String Utilities
```rust
use hiivelabs_rand_utils_lib::prelude::*;

// Convert strings between cases
let title = convert_str_to_title_case("hello world");
let underscore = convert_str_to_underscore_case("HelloWorld");
```

#### Weighted Selection
```rust
use hiivelabs_rand_utils_lib::prelude::*;

// Create a weighted picker for probability-based selection
let mut picker = WeightedPicker::new();
picker.add_item("common", 70);
picker.add_item("rare", 20);
picker.add_item("legendary", 10);

let selected = picker.pick();
```

#### Worker Pool
```rust
use hiivelabs_rand_utils_lib::prelude::*;

// Create and use a worker pool for parallel processing
let pool = create_worker_pool(4); // 4 worker threads

// Submit tasks to the pool
let task = Task::new(|| {
    // Your processing logic here
});

submit_message_to_worker_pool(&pool, WorkerPoolMessage::Execute(task));

// Don't forget to shutdown when done
shutdown_worker_pool(pool);
```

#### Seed Generation
```rust
use hiivelabs_rand_utils_lib::prelude::*;

// Create deterministic seeds from byte data
let seed_data = b"my_seed_data";
let seed = create_seed_from_bytes(seed_data);
```

## Module Structure

- **Grid Utilities**: Coordinate system types and operations
- **Seed Utilities**: Deterministic random seed creation
- **String Utilities**: Text manipulation and case conversion
- **Weighted Picker**: Probability-based item selection
- **Worker Pool**: Multi-threaded task execution system
  - Task definitions and management
  - Worker pool implementation
  - Message passing system

## Version

Current version: 0.1.0

## Features

- WebAssembly compatible random number generation
- Thread-safe worker pool implementation
- Type-safe coordinate system
- Configurable logging system

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

This project uses the Rust 2021 edition.