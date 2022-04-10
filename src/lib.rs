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
#![warn(missing_docs)]
#![warn(clippy::all)]
#![deny(clippy::correctness)]
#![forbid(unsafe_code)]
// core brain flak macro, @() is the <> instead, as well as greedy ! ... to
// indicate it still contains <...>
#[doc(hidden)]
#[macro_export]
macro_rules! internal_simple_eval {
    (($left:ident, $right:ident) ()) => { 0 };
    (($left:ident, $right:ident) (()$($code:tt)*)) => {{
        let rest = $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        };
        rest + 1
    }};
    (($left:ident, $right:ident) ([]$($code:tt)*)) => {{
        use std::vec::Vec;
        let len = Vec::len($left);
        let len = core::convert::TryInto::try_into(len);
        let len = core::result::Result::unwrap(len);
        // HACK: this is to infer len to have similar type as the element
        if false {
            Vec::push($left, len);
        }
        let rest = $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        };
        rest + len
    }};
    (($left:ident, $right:ident) ({}$($code:tt)*)) => {{
        let popped = std::vec::Vec::pop($left);
        let popped = core::option::Option::unwrap_or_default(popped);
        let rest = $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        };
        rest + popped
    }};
    (($left:ident, $right:ident) (@()$($code:tt)*)) => {{
        core::mem::swap($left, $right);
        $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) (($($first:tt)+)$($code:tt)*)) => {{
        let num = $crate::internal_simple_eval! {
            ($left, $right)
            ($($first)+)
        };
        std::vec::Vec::push($left, num);
        let rest = $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        };
        rest + num
    }};
    (($left:ident, $right:ident) ([$($first:tt)+]$($code:tt)*)) => {{
        let num = $crate::internal_simple_eval! {
            ($left, $right)
            ($($first)*)
        };
        let rest = $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        };
        rest - num
    }};
    (($left:ident, $right:ident) ({$($first:tt)+}$($code:tt)*)) => {{
        let mut num = 0;
        while let core::option::Option::Some(top) = <[_]>::last($left) {
            if *top == 0 {
                break;
            } else {
                num += $crate::internal_simple_eval! {
                    ($left, $right)
                    ($($first)+)
                };
            }
        }
        let rest = $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        };
        rest + num
    }};
    (($left:ident, $right:ident) (@($($first:tt)+)$($code:tt)*)) => {{
        $crate::internal_simple! {
            ($left, $right)
            ($($first)*)
        }
        $crate::internal_simple_eval! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) (!$($code:tt)*)) => {
        $crate::internal! {
            ($left, $right, internal_simple_eval)
            (())
            ($($code)*)
        }
    };
}
// same as above, but discards the return value as possible.
// necessary to avoid "unused" warnings.
#[doc(hidden)]
#[macro_export]
macro_rules! internal_simple {
    (($left:ident, $right:ident) ()) => { () };
    (($left:ident, $right:ident) (()$($code:tt)*)) => {
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    };
    (($left:ident, $right:ident) ([]$($code:tt)*)) => {
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    };
    (($left:ident, $right:ident) ({}$($code:tt)*)) => {{
        std::vec::Vec::pop($left);
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) (@()$($code:tt)*)) => {{
        core::mem::swap($left, $right);
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) (($($first:tt)+)$($code:tt)*)) => {{
        let num = $crate::internal_simple_eval! {
            ($left, $right)
            ($($first)+)
        };
        std::vec::Vec::push($left, num);
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) ([$($first:tt)+]$($code:tt)*)) => {{
        $crate::internal_simple!{
            ($left, $right)
            ($($first)*)
        }
        $crate::internal_simple!{
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) ({$($first:tt)+}$($code:tt)*)) => {{
        while let core::option::Option::Some(top) = <[_]>::last($left) {
            if *top == 0 {
                break;
            } else {
                $crate::internal_simple! {
                    ($left, $right)
                    ($($first)+)
                }
            }
        }
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) (@($($first:tt)+)$($code:tt)*)) => {{
        $crate::internal_simple! {
            ($left, $right)
            ($($first)*)
        }
        $crate::internal_simple! {
            ($left, $right)
            ($($code)*)
        }
    }};
    (($left:ident, $right:ident) (!$($code:tt)*)) => {
        $crate::internal! {
            ($left, $right, internal_simple)
            (())
            ($($code)*)
        }
    };
}
// another brain flak macro that deals with <...>
// this internally replaces <...> with @(...) so it can be invoked with
// internal_simple or internal_simple_eval
// this works with simple token stack to deal with nested <...>
#[doc(hidden)]
#[macro_export]
macro_rules! internal {
    (($left:ident, $right:ident, $macro:ident) (($($first:tt)*)) ()) => {
        $crate::$macro! {
            ($left, $right)
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
/// This macro accepts a single expression with type `&mut Vec<T>` where `T`
/// is any numeric types followed by `=>` then the Brain-Flak code. After the
/// macro invocation, the passed value is then left with the active stack as if
/// it is the output.
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
///
/// More information about Brain-Flak can be found on
/// [Esolang wiki page](https://esolangs.org/wiki/Brain-Flak), as well as on
/// its [GitHub repository](https://github.com/DJMcMayhem/Brain-Flak)
#[macro_export]
macro_rules! brain_flak {
    ($input:expr $(,)? => $($code:tt)*) => {{
        use std::vec::Vec;
        let left: &mut Vec<_> = $input;
        let mut right = Vec::new();
        // HACK: this is to infer right is the same type as left
        if false {
            let item = Vec::pop(&mut right);
            let item = core::option::Option::unwrap(item);
            Vec::push(left, item);
        }
        #[allow(unused)]
        let right = &mut right;
        $crate::internal! {
            (left, right, internal_simple)
            (())
            ($($code)*)
        }
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
