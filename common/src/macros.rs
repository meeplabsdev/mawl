#[macro_export]
macro_rules! b {
    ($x:expr) => {
        $x != 0
    };
}

#[macro_export]
macro_rules! STRUCT {
    (
        $name:ident {
            $($field:ident : $typ:ty),* $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Copy)]
        pub struct $name {
            $(pub $field: $typ),*
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl Default for $name {
            fn default() -> Self {
                unsafe { core::mem::zeroed() }
            }
        }

        pastey::paste! {
            pub type [<P_ $name>] = *mut $name;
            pub type [<PC_ $name>] = *const $name;
        }
    };
}

#[macro_export]
macro_rules! UNION {
    (
        $name:ident {
            $($field:ident : $typ:ty),* $(,)?
        }
    ) => {
        #[repr(C)]
        #[derive(Clone, Copy)]
        pub union $name {
            $(pub $field: $typ),*
        }

        pastey::paste! {
            pub type [<P_ $name>] = *mut $name;
            pub type [<PC_ $name>] = *const $name;
        }
    };
}

#[macro_export]
macro_rules! ENUM {
    (
        $name:ident {
            $($field:ident $(= $value:expr)?),* $(,)?
        }
    ) => {
        #[repr(i32)]
        #[derive(Clone, Copy)]
        pub enum $name {
            $(
                $field $(= $value)?
            ),*
        }
    };
}

#[macro_export]
macro_rules! TYPE {
    ($name:ident = $base:ty) => {
        pastey::paste! {
            pub type $name = $base;
            pub type [<P_ $name>] = *mut $name;
            pub type [<PC_ $name>] = *const $name;
        }
    };
}
