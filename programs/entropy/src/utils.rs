use crate::*;
/// Adds a LEN constant to an account struct, with specified value.
/// This is used because space must now be explicitly defined for accounts
/// when using `init`
#[macro_export]
macro_rules! size {
    ($name: ident, $size:expr) => {
        impl $name {
            pub const LEN: usize = $size + 8;
        }
    };
}

pub fn get_pubkey(address: &str) -> Pubkey {
    return Pubkey::from_str(address).unwrap()
}