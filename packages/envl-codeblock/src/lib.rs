// Respect quote (https://github.com/dtolnay/quote)

extern crate alloc;

#[path = "runtime.rs"]
pub mod __private;

#[doc(hidden)]
pub use quote;

#[macro_export]
macro_rules! code_block {
    () => {
        $crate::__private::TokenStream::new()
    };

    ($tt:tt) => {{
        let mut _s = $crate::__private::TokenStream::new();
        $crate::push_token!{$tt _s}
        _s
    }};

    (# $var:ident) => {{
        let mut _s = $crate::__private::TokenStream::new();
        $crate::quote::ToTokens::to_tokens(&$var, &mut _s);
        _s
    }};
    ($tt1:tt $tt2:tt) => {{
        let mut _s = $crate::__private::TokenStream::new();
        $crate::push_token!{$tt1 _s}
        $crate::push_token!{$tt2 _s}
        _s
    }};

    ($($tts:tt)*) => {{
        let mut _s = $crate::__private::TokenStream::new();
        $crate::each_token!(_s $($tts)*);
        _s
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! pounded_var_names {
    ($call:ident! $extra:tt $($tts:tt)*) => {
        $crate::pounded_var_names_with_context!{$call! $extra
            (@ $($tts)*)
            ($($tts)* @)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pounded_var_names_with_context {
    ($call:ident! $extra:tt ($($b1:tt)*) ($($curr:tt)*)) => {
        $(
            $crate::pounded_var_with_context!{$call! $extra $b1 $curr}
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! pounded_var_with_context {
    ($call:ident! $extra:tt $b1:tt ( $($inner:tt)* )) => {
        $crate::pounded_var_names!{$call! $extra $($inner)*}
    };

    ($call:ident! $extra:tt $b1:tt [ $($inner:tt)* ]) => {
        $crate::pounded_var_names!{$call! $extra $($inner)*}
    };

    ($call:ident! $extra:tt $b1:tt { $($inner:tt)* }) => {
        $crate::pounded_var_names!{$call! $extra $($inner)*}
    };

    ($call:ident!($($extra:tt)*) # $var:ident) => {
        $crate::$call!($($extra)* $var);
    };

    ($call:ident! $extra:tt $b1:tt $curr:tt) => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_into_iter {
    ($has_iter:ident $var:ident) => {
        #[allow(unused_mut)]
        let (mut $var, i) = $var.quote_into_iter();
        let $has_iter = $has_iter | i;
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! bind_next_or_break {
    ($var:ident) => {
        let $var = match $var.next() {
            Some(_x) => $crate::__private::RepInterp(_x),
            None => break,
        };
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! each_token {
    ($tokens:ident $($tts:tt)*) => {
        $crate::tokens_with_context!{$tokens
            (@ @ @ @ @ @ $($tts)*)
            (@ @ @ @ @ $($tts)* @)
            (@ @ @ @ $($tts)* @ @)
            (@ @ @ $(($tts))* @ @ @)
            (@ @ $($tts)* @ @ @ @)
            (@ $($tts)* @ @ @ @ @)
            ($($tts)* @ @ @ @ @ @)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! tokens_with_context {
    ($tokens:ident
        ($($b3:tt)*) ($($b2:tt)*) ($($b1:tt)*)
        ($($curr:tt)*)
        ($($a1:tt)*) ($($a2:tt)*) ($($a3:tt)*)
    ) => {
        $(
            $crate::token_with_context!{$tokens $b3 $b2 $b1 $curr $a1 $a2 $a3}
        )*
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! token_with_context {
    ($tokens:ident $b3:tt $b2:tt $b1:tt @ $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) ( $($inner:tt)* ) * $a3:tt) => {{
        use $crate::__private::ext::*;
        let has_iter = $crate::__private::HasIterator::<false>;
        $crate::pounded_var_names!{bind_into_iter!(has_iter) () $($inner)*}
        <_ as $crate::__private::CheckHasIterator<true>>::check(has_iter);

        while true {
            $crate::pounded_var_names!{bind_next_or_break!() () $($inner)*}
            $crate::each_token!{$tokens $($inner)*}
        }
    }};

    ($tokens:ident $b3:tt $b2:tt # (( $($inner:tt)* )) * $a2:tt $a3:tt) => {};
    ($tokens:ident $b3:tt # ( $($inner:tt)* ) (*) $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) ( $($inner:tt)* ) $sep:tt *) => {{
        use $crate::__private::ext::*;
        let mut _i = 0usize;
        let has_iter = $crate::__private::HasIterator::<false>;
        $crate::pounded_var_names!{bind_into_iter!(has_iter) () $($inner)*}
        <_ as $crate::__private::CheckHasIterator<true>>::check(has_iter);

        while true {
            $crate::pounded_var_names!{bind_next_or_break!() () $($inner)*}
            if _i > 0 {
                $crate::push_token!{$sep $tokens}
            }
            _i += 1;
            $crate::each_token!{$tokens $($inner)*}
        }
    }};

    ($tokens:ident $b3:tt $b2:tt # (( $($inner:tt)* )) $sep:tt * $a3:tt) => {};
    ($tokens:ident $b3:tt # ( $($inner:tt)* ) ($sep:tt) * $a2:tt $a3:tt) => {};

    ($tokens:ident # ( $($inner:tt)* ) * (*) $a1:tt $a2:tt $a3:tt) => {
        $crate::push_token!{* $tokens};
    };

    ($tokens:ident # ( $($inner:tt)* ) $sep:tt (*) $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt (#) $var:ident $a2:tt $a3:tt) => {
        $crate::quote::ToTokens::to_tokens(&$var, &mut $tokens);
    };

    ($tokens:ident $b3:tt $b2:tt # ($var:ident) $a1:tt $a2:tt $a3:tt) => {};

    ($tokens:ident $b3:tt $b2:tt $b1:tt ($curr:tt) $a1:tt $a2:tt $a3:tt) => {
        $crate::push_token! {$curr $tokens}
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! push_token {
    (( $($inner:tt)* ) $tokens:ident) => {
        $crate::__private::push_group(
            &mut $tokens,
            $crate::__private::Delimiter::Parenthesis,
            $crate::code_block!($($inner)*),
        );
    };

    ([ $($inner:tt)* ] $tokens:ident) => {
        $crate::__private::push_group(
            &mut $tokens,
            $crate::__private::Delimiter::Bracket,
            $crate::code_block!($($inner)*),
        );
    };

    ({ $($inner:tt)* } $tokens:ident) => {
        $crate::__private::push_group(
            &mut $tokens,
            $crate::__private::Delimiter::Brace,
            $crate::code_block!($($inner)*),
        );
    };

    ($token:tt $tokens:ident) => {
        $crate::__private::push_ident(&mut $tokens, stringify!($token).to_string());
    };
}
