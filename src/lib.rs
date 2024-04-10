pub mod random;

pub mod prelude {
    // exports
    pub use crate::random::seed_utils_impl::{
        create_seed_from_bytes, create_seed_from_guid_bytes_x_y, create_seed_from_guid_x_y,
    };
}
