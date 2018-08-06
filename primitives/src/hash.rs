macro_rules! impl_hash {
    ($name:ident, $size:expr) => {
        pub struct $name([u8; $size]);

        impl Default for $name {
            fn default() -> Self {
                $name([0u8; $size])
            }
        }
    };
}

impl_hash!(H256, 32);
