/*
    Appellation: fmt <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(hidden)]
#[macro_export]
macro_rules! display {
    (json: $($type:ty),* $(,)?) => {
       $($crate::display!(@json $type);)*
    };
    (@json $type:ty) => {
        impl ::core::fmt::Display for $type {
            $crate::display!(@json);
        }
    };
    (@json $name:ident<$($T:ident),* $(,)?> where $($rest:tt)*) => {
        impl<$($T),*> ::core::fmt::Display for $name<$($T),*> where $($rest)* {
            $crate::display!(@json);
        }
    };
    (@json) => {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            write!(
                f,
                "{}",
                serde_json::to_string(self).unwrap()
            )
        }
    };
}
