macro_rules! derive_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(u8);

        impl $name {
            pub fn as_usize(self) -> usize {
                self.0 as usize
            }

            pub fn from_usize(i: usize) -> Self {
                $name(u8::try_from(i).unwrap())
            }
        }
    };
}

derive_id! {SiteId}
