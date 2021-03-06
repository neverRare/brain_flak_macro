//! Stack manipulation with [Brain-Flak](https://esolangs.org/wiki/Brain-Flak).
//!
//! ```
//! use brain_flak_macro::brain_flak;
//!
//! let mut vec = vec![20, 5];
//! brain_flak! { &mut vec =>
//!     // multiplication
//!     ([({}<([({}(<()>))<>](<()>))<>>)<>]){({}[()]<(({})<({}{})>)>)<>}{}{}<>{}{}{}<>
//! }
//! assert_eq!(vec![100], vec);
//! ```
//!
//! You can pass at most 2 and at least 1 mutable reference to vector for it's
//! input. Followed by `=>` then the Brain-Flak code.
#![warn(missing_docs)]
// core brain flak macro, @() is the <> instead, as well as greedy ! ... to
// indicate it still contains <...>
#[doc(hidden)]
#[macro_export]
macro_rules! internal_simple_eval {
    // $stack is an array of 2 stacks
    // $active is a usize either 0 or 1
    // these should be an identifier
    (($stack:ident, $active:ident) ()) => { 0 };
    (($stack:ident, $active:ident) (()$($code:tt)*)) => {{
        let rest = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        };
        rest + 1
    }};
    (($stack:ident, $active:ident) ([]$($code:tt)*)) => {{
        use std::vec::Vec;
        let len = Vec::len($stack[$active]);
        let len = core::convert::TryInto::try_into(len);
        let len = core::result::Result::unwrap(len);
        // HACK: this is to infer len to have similar type as the element
        Vec::push($stack[$active], len);
        Vec::pop($stack[$active]);
        let rest = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        };
        rest + len
    }};
    (($stack:ident, $active:ident) ({}$($code:tt)*)) => {{
        let popped = std::vec::Vec::pop($stack[$active]);
        let popped = core::option::Option::unwrap_or_default(popped);
        let rest = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        };
        rest + popped
    }};
    (($stack:ident, $active:ident) (@()$($code:tt)*)) => {{
        $active = 1 - $active;
        $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) (($($first:tt)+)$($code:tt)*)) => {{
        let num = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($first)+)
        };
        std::vec::Vec::push($stack[$active], num);
        let rest = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        };
        rest + num
    }};
    (($stack:ident, $active:ident) ([$($first:tt)+]$($code:tt)*)) => {{
        let num = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($first)*)
        };
        let rest = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        };
        rest - num
    }};
    (($stack:ident, $active:ident) ({$($first:tt)+}$($code:tt)*)) => {{
        let mut num = 0;
        while let core::option::Option::Some(top) = <[_]>::last($stack[$active]) {
            if *top == 0 {
                break;
            } else {
                num += $crate::internal_simple_eval! {
                    ($stack, $active)
                    ($($first)+)
                };
            }
        }
        let rest = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        };
        rest + num
    }};
    (($stack:ident, $active:ident) (@($($first:tt)+)$($code:tt)*)) => {{
        $crate::internal_simple! {
            ($stack, $active)
            ($($first)*)
        }
        $crate::internal_simple_eval! {
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) (!$($code:tt)*)) => {
        $crate::internal! {
            ($stack, $active, internal_simple_eval)
            (())
            ($($code)*)
        };
    };
}
// same as above, but discards the return value as possible.
// necessary to avoid "unused" warnings.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_simple {
    (($stack:ident, $active:ident) ()) => { () };
    (($stack:ident, $active:ident) (()$($code:tt)*)) => {
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        }
    };
    (($stack:ident, $active:ident) ([]$($code:tt)*)) => {
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        }
    };
    (($stack:ident, $active:ident) ({}$($code:tt)*)) => {{
        std::vec::Vec::pop($stack[$active]);
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) (@()$($code:tt)*)) => {{
        $active = 1 - $active;
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) (($($first:tt)+)$($code:tt)*)) => {{
        let num = $crate::internal_simple_eval! {
            ($stack, $active)
            ($($first)+)
        };
        std::vec::Vec::push($stack[$active], num);
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) ([$($first:tt)+]$($code:tt)*)) => {{
        $crate::internal_simple!{
            ($stack, $active)
            ($($first)*)
        }
        $crate::internal_simple!{
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) ({$($first:tt)+}$($code:tt)*)) => {{
        while let core::option::Option::Some(top) = <[_]>::last($stack[$active]) {
            if *top == 0 {
                break;
            } else {
                $crate::internal_simple! {
                    ($stack, $active)
                    ($($first)+)
                };
            }
        }
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        };
    }};
    (($stack:ident, $active:ident) (@($($first:tt)+)$($code:tt)*)) => {{
        $crate::internal_simple! {
            ($stack, $active)
            ($($first)*)
        }
        $crate::internal_simple! {
            ($stack, $active)
            ($($code)*)
        }
    }};
    (($stack:ident, $active:ident) (!$($code:tt)*)) => {{
        $crate::internal! {
            ($stack, $active, internal_simple)
            (())
            ($($code)*)
        }
    }};
}
// another brain flak macro that deals with <...>
// this internally replaces <...> with @(...) so it can be invoked with
// internal_simple
// this works with simple token stack to deal with nested <...>
#[doc(hidden)]
#[macro_export]
macro_rules! internal {
    (($stack:ident, $active:ident, $macro:ident) (($($first:tt)*)) ()) => {
        $crate::$macro! {
            ($stack, $active)
            ($($first)*)
        }
    };
    (($($meta:tt)*) ($($rest:tt)*) (<<$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (()()$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) ($($rest:tt)*) (<$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (()$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)($($second:tt)*)($($third:tt)*)$($rest:tt)*) (>>$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($third)*@($($second)*@($($first)*)))$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)($($second:tt)*)$($rest:tt)*) (>$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($second)*@($($first)*))$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)$($rest:tt)*) (()$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($first)*())$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)$($rest:tt)*) ([]$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($first)*[])$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)$($rest:tt)*) ({}$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($first)*{})$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)$($rest:tt)*) (($($token:tt)+)$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($first)*(!$($token)+))$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)$($rest:tt)*) ([$($token:tt)+]$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($first)*[!$($token)+])$($rest)*)
            ($($code)*)
        }
    };
    (($($meta:tt)*) (($($first:tt)*)$($rest:tt)*) ({$($token:tt)+}$($code:tt)*)) => {
        $crate::internal! {
            ($($meta)*)
            (($($first)*{!$($token)+})$($rest)*)
            ($($code)*)
        }
    };
}
/// Stack manipulation with [Brain-Flak](https://esolangs.org/wiki/Brain-Flak).
///
/// # Brain-Flak reference table
///
/// | Nilad | Return value                 | Action                                  |
/// | :---: | ---------------------------- | --------------------------------------- |
/// | `()`  | +1                           | None                                    |
/// | `[]`  | Height of active stack       | None                                    |
/// | `{}`  | Value of top of active stack | Pops the top value off the active stack |
/// | `<>`  | 0                            | Switches the active stack               |
///
/// |   Monad   | Return value                                   | Action                                                         |
/// | :-------: | ---------------------------------------------- | -------------------------------------------------------------- |
/// | `(`...`)` | Inside value                                   | Pushes the inside value to the top of the active stack         |
/// | `[`...`]` | Negative inside value                          | None                                                           |
/// | `{`...`}` | Sum of the inside values across all executions | Executes the inside while the top of the inside stack is not 0 |
/// | `<`...`>` | 0                                              | None                                                           |
///
/// The table above is shamelessly copied from
/// <https://github.com/DJMcMayhem/Brain-Flak/wiki/Reference>.
/// More information about Brain-Flak can be found on
/// [Esolang wiki page](https://esolangs.org/wiki/Brain-Flak), as well as on
/// its [GitHub repository](https://github.com/DJMcMayhem/Brain-Flak)
///
/// Refer to the [crate document](./index.html) for more information about the
/// macro.
#[macro_export]
macro_rules! brain_flak {
    ($left:expr, $right:expr $(,)? => $($code:tt)*) => {{
        use std::vec::Vec;
        let left: &mut Vec<_> = $left;
        let right: &mut Vec<_> = $right;
        let stacks = [left, right];
        #[allow(unused_mut)]
        let mut active = 0;
        $crate::internal! {
            (stacks, active, internal_simple)
            (())
            ($($code)*)
        }
    }};
    ($input:expr $(,)? => $($code:tt)*) => {{
        let mut right = std::vec::Vec::new();
        $crate::brain_flak! { $input, &mut right =>
            $($code)*
        };
    }};
}
#[cfg(test)]
mod test {
    use super::brain_flak;
    #[test]
    fn zero() {
        let mut vec = vec![];
        brain_flak! { &mut vec =>
            (<()>)
        }
        assert_eq!(vec![0], vec);
    }
    #[test]
    fn add() {
        let mut vec = vec![10, 20];
        brain_flak! { &mut vec =>
            ({}{})
        }
        assert_eq!(vec![30], vec);
    }
    #[test]
    fn sum() {
        let mut vec = vec![10, 20, 30];
        brain_flak! { &mut vec =>
            (([]){[{}]{}([])}{})
        }
        assert_eq!(vec![60], vec);
    }
    #[test]
    fn subtract() {
        let mut vec = vec![20, 5];
        brain_flak! { &mut vec =>
            ([{}]{})
        }
        assert_eq!(vec![15], vec);
    }
    #[test]
    fn multiply() {
        let mut vec = vec![20, 5];
        brain_flak! { &mut vec =>
            ([({}<([({}(<()>))<>](<()>))<>>)<>]){({}[()]<(({})<({}{})>)>)<>}{}{}<>{}{}{}<>
        }
        assert_eq!(vec![100], vec);
    }
    #[test]
    fn sort() {
        let mut vec = vec![2, 3, 5, 1, 4];
        brain_flak! { &mut vec =>
            ([]){({}[()]<(([])<{({}[()]<([([({}<(({})<>)<>>)<><({}<>)>]{}<(())>)
            ](<>)){({}())<>}{}({}<><{}{}>){{}<>(<({}<({}<>)<>>)<>({}<>)>)}{}({}<
            >)<>>)}{}<>{}>[()]){({}[()]<({}<>)<>>)}{}<>>)}{}([]){((({}[()])<{({}
            [()]<({}<({}<>)<>>)>)}{}>)<{({}[()]<<>({}<>)>)}{}>)}{}
        }
        assert_eq!(vec![1, 2, 3, 4, 5], vec);
    }
}
