//! # try_utils
//!
//! A small collections of macros for adding try guards in rust
//!
//! ## Examples
//!
//! ### `try_utils::try_return`
//!
//! ```rust
//! use try_utils::try_return;
//!
//! fn my_func1(val: Option<i32>) -> i32 {
//!     let val = try_return!(val, {
//!         // do some expensive computation here
//!         panic!()
//!     });
//!     val
//! }
//!
//! assert_eq!(my_func1(Some(10)), 10); // the expensive computation will not be run
//!
//! fn my_func2(val: Option<i32>) -> i32 {
//!     let val = try_return!(val, 1234);
//!     val
//! }
//! assert_eq!(my_func2(Some(10)), 10);
//! assert_eq!(my_func2(None), 1234);
//!
//! fn my_func3(val: Option<i32>) {
//!     let _ = try_return!(val);
//!     panic!();
//! }
//! my_func3(None);
//! ```
//!
//! ### `try_utils::try_continue`
//! ```rust
//! use try_utils::try_continue;
//!
//! 'label: for _ in 0..10 {
//!     for _ in 0..10 {
//!         let _: u32 = try_continue!(None, 'label);
//!         panic!();
//!     }
//!     panic!();
//! }
//!
//! for _ in 0..10 {
//!     let val: u32 = try_continue!(Some(10));
//!     assert_eq!(val, 10);
//! }
//! ```
//!
//! ### `try_utils::try_break`
//! ```rust
//! use try_utils::try_break;
//!
//! 'label: for _ in 0..10 {
//!     for _ in 0..10 {
//!         let _: u32 = try_break!(None, 'label);
//!         panic!();
//!     }
//!     panic!();
//! }
//!
//! for _ in 0..10 {
//!     let val: u32 = try_break!(Some(10));
//!     assert_eq!(val, 10);
//! }
//! ```

/// A trait for converting a type to an option to use in try_utils macros
pub trait TryAsOption {
    type Output;
    /// Converts this type to an option
    fn try_as_option(self) -> Option<Self::Output>;
}

impl<T> TryAsOption for Option<T> {
    type Output = T;
    fn try_as_option(self) -> Option<Self::Output> {
        self
    }
}

impl<T, E> TryAsOption for Result<T, E> {
    type Output = T;
    fn try_as_option(self) -> Option<Self::Output> {
        self.ok()
    }
}

/// Returns the value of an expression if it is `Some` or `Ok`, otherwise
/// returns from the current function with the given value or `()` if none is
/// given.
///
/// ```
/// use try_utils::try_return;
///
/// fn my_func(val: Option<i32>) -> i32 {
///     let val = try_return!(val, 1234);
///     val
/// }
/// assert_eq!(my_func(Some(10)), 10);
/// assert_eq!(my_func(None), 1234);
/// ```
#[macro_export]
macro_rules! try_return {
    ($e: expr) => {
        try_return!($e, ());
    };

    ($e: expr, $ret: expr) => {{
        use $crate::TryAsOption;
        match $e.try_as_option() {
            Some(v) => v,
            None => return $ret,
        }
    }};
}

/// Returns the value of an expression if it is `Some` or `Ok`, otherwise
/// continues the current loop.
///
/// An optional label can be given to continue a loop with a label.
///
/// ```
/// use try_utils::try_continue;
///
/// 'label: for _ in 0..10 {
///     for _ in 0..10 {
///         let _: u32 = try_continue!(None, 'label);
///         panic!();
///     }
///     panic!();
/// }
///
/// for _ in 0..10 {
///     let val: u32 = try_continue!(Some(10));
///     assert_eq!(val, 10);
/// }
/// ```
#[macro_export]
macro_rules! try_continue {
    ($e: expr) => {{
        use $crate::TryAsOption;
        match $e.try_as_option() {
            Some(v) => v,
            None => continue,
        }
    }};

    ($e: expr, $label: lifetime) => {{
        use $crate::TryAsOption;
        match $e.try_as_option() {
            Some(v) => v,
            None => continue $label,
        }
    }};
}

/// Returns the value of an expression if it is `Some` or `Ok`, otherwise
/// breaks the current loop.
///
/// An optional label can be given to continue a loop with a label.
///
/// ```
/// use try_utils::try_break;
///
/// 'label: for _ in 0..10 {
///     for _ in 0..10 {
///         let _: u32 = try_break!(None, 'label);
///         panic!();
///     }
///     panic!();
/// }
///
/// for _ in 0..10 {
///     let val: u32 = try_break!(Some(10));
///     assert_eq!(val, 10);
/// }
/// ```
#[macro_export]
macro_rules! try_break {
    ($e: expr) => {{
        use $crate::TryAsOption;
        match $e.try_as_option() {
            Some(v) => v,
            None => break,
        }
    }};

    ($e: expr, $label: lifetime) => {{
        use $crate::TryAsOption;
        match $e.try_as_option() {
            Some(v) => v,
            None => break $label,
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn try_return_none() {
        fn return_one_option() -> i32 {
            let _: u32 = try_return!(None, 1);
            0
        }

        fn return_one_result() -> i32 {
            let _: u32 = try_return!(Err(10), 1);
            0
        }

        assert_eq!(return_one_option(), 1);
        assert_eq!(return_one_result(), 1);
    }

    #[test]
    fn try_return_some() {
        fn return_zero_option() -> i32 {
            let val: u32 = try_return!(Some(10), 1);
            assert_eq!(val, 10);
            0
        }

        fn return_zero_result() -> i32 {
            let val: u32 = try_return!(Ok::<_, u64>(10), 1);
            assert_eq!(val, 10);
            0
        }

        assert_eq!(return_zero_option(), 0);
        assert_eq!(return_zero_result(), 0);
    }

    #[test]
    fn try_continue_none() {
        let mut count = 0;
        for _ in 0..10 {
            count += 1;
            let _: u32 = try_continue!(None);
            panic!();
        }
        assert_eq!(count, 10);

        let mut count = 0;
        'outer: for _ in 0..10 {
            count += 1;
            loop {
                let _: u32 = try_continue!(None, 'outer);
                panic!();
            }
        }
        assert_eq!(count, 10);
    }

    #[test]
    fn try_continue_some() {
        let mut count = 0;
        for _ in 0..10 {
            count += 1;
            let val: u32 = try_continue!(Some(10));
            assert_eq!(val, 10)
        }
        assert_eq!(count, 10);

        let mut count = 0;
        'outer: for _ in 0..10 {
            count += 1;
            for _ in 0..10 {
                let val: u32 = try_continue!(Some(10), 'outer);
                assert_eq!(val, 10);
            }
        }
        assert_eq!(count, 10);
    }

    #[test]
    fn try_break_none() {
        loop {
            let _: u32 = try_break!(None);
            panic!();
        }

        'outer: loop {
            for _ in 0..10 {
                let _: u32 = try_break!(None, 'outer);
                panic!();
            }
            panic!();
        }
    }

    #[test]
    fn try_break_some() {
        for _ in 0..10 {
            let val: u32 = try_break!(Some(10));
            assert_eq!(val, 10)
        }

        'outer: for _ in 0..10 {
            for _ in 0..10 {
                let val: u32 = try_break!(Some(10), 'outer);
                assert_eq!(val, 10)
            }
        }
    }
}
