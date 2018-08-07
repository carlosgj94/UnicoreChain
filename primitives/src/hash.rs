macro_rules! impl_hash {
    ($name:ident, $size:expr) => {
        pub struct $name([u8; $size]);

        impl Default for $name {
            fn default() -> Self {
                $name([0u8; $size])
            }
        }

        impl From<$name> for [u8; $size] {
            fn from(h: $name) -> Self {
                h.0
            }
        }
    };
}

impl_hash!(H256, 32);
