// auto-generated: "lalrpop 0.19.8"
// sha3: 66de09c86488683b949dfaf5fd62cc2ac091728b29dae43d0e075828df6de250
use std::str::FromStr;
use std::collections::HashMap;
use lalrpop_util::ParseError;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Statement {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use std::str::FromStr;
    use std::collections::HashMap;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use self::__lalrpop_util::lexer::Token;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(f64),
        Variant2(String),
    }
    const __ACTION: &[i8] = &[
        // State 0
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 1
        -19, 0, -19, -19, -19, -19, -19, -19, 0, -19, 0, 0, 0, 0, 0, 0, 0, 0, -19, 0, 0, 37,
        // State 2
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 3
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 4
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 5
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 6
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 7
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 8
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 9
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 10
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 11
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 12
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 13
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 14
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 15
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 16
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 17
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 18
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 19
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 20
        3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 36, 37,
        // State 21
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 22
        0, 0, -8, 0, 0, -8, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 37,
        // State 23
        30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 24
        0, 0, -6, 0, 0, -6, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, 0, 0, 37,
        // State 25
        -16, 0, -16, -16, -16, -16, -16, -16, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, 37,
        // State 26
        -15, 0, -15, -15, -15, -15, -15, -15, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, 37,
        // State 27
        -17, 0, -17, -17, -17, -17, -17, -17, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, 37,
        // State 28
        -14, 0, -14, -14, -14, -14, -14, -14, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0, 0, 37,
        // State 29
        3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 36, 37,
        // State 30
        0, 0, 0, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        17, 0, -13, 18, 19, -13, -13, 20, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, -13, 0, 0, 0,
        // State 32
        -24, 0, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, -24, 0, 0, -24,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        -27, 0, 0, -27, -27, -27, -27, -27, 22, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27,
        // State 35
        -20, 0, -20, -20, -20, -20, -20, -20, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, -20, 0, 0, -20,
        // State 36
        -23, 0, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0, -23, 0, 0, -23,
        // State 37
        -28, 0, -28, -28, -28, -28, -28, -28, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, -28,
        // State 38
        0, 0, 53, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        -27, 0, -27, -27, -27, -27, -27, -27, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, -27,
        // State 40
        0, 0, 54, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 55, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 56, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 57, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 58, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 59, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 60, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 0, 0, 0,
        // State 48
        17, 0, -1, 18, 19, -1, -1, 20, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0,
        // State 49
        17, 0, -2, 18, 19, -2, -2, 20, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, -2, 0, 0, 0,
        // State 50
        0, 0, 62, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        -25, 0, -25, -25, -25, -25, -25, -25, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0, 0, -25,
        // State 53
        0, 0, -9, 0, 0, -9, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, 0,
        // State 54
        0, 0, -4, 0, 0, -4, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0,
        // State 55
        0, 0, -12, 0, 0, -12, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, 0, 0, 0,
        // State 56
        0, 0, -10, 0, 0, -10, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0, 0, 0,
        // State 57
        0, 0, -3, 0, 0, -3, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -3, 0, 0, 0,
        // State 58
        0, 0, -7, 0, 0, -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0,
        // State 59
        0, 0, -5, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0,
        // State 60
        -26, 0, -26, -26, -26, -26, -26, -26, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, -26,
        // State 61
        -18, 0, -18, -18, -18, -18, -18, -18, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, 0,
        // State 62
        0, 0, 64, 0, 0, 15, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, -11, 0, 0, -11, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 22 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -19,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -8,
        // State 23
        0,
        // State 24
        -6,
        // State 25
        -16,
        // State 26
        -15,
        // State 27
        -17,
        // State 28
        -14,
        // State 29
        0,
        // State 30
        -21,
        // State 31
        -13,
        // State 32
        -24,
        // State 33
        -29,
        // State 34
        -27,
        // State 35
        -20,
        // State 36
        -23,
        // State 37
        -28,
        // State 38
        0,
        // State 39
        -27,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        -1,
        // State 49
        -2,
        // State 50
        0,
        // State 51
        -22,
        // State 52
        -25,
        // State 53
        -9,
        // State 54
        -4,
        // State 55
        -12,
        // State 56
        -10,
        // State 57
        -3,
        // State 58
        -7,
        // State 59
        -5,
        // State 60
        -26,
        // State 61
        -18,
        // State 62
        0,
        // State 63
        -11,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                2 => 38,
                4 => 40,
                5 => 41,
                6 => 42,
                8 => 43,
                9 => 44,
                10 => 45,
                11 => 46,
                12 => 47,
                16 => 50,
                21 => 51,
                29 => 62,
                _ => 30,
            },
            1 => match state {
                14 => 48,
                15 => 49,
                _ => 31,
            },
            2 => 32,
            3 => 33,
            4 => match state {
                0 => 34,
                2..=21 | 29 => 39,
                _ => 37,
            },
            5 => match state {
                3 => 22,
                7 => 23,
                13 => 24,
                17 => 25,
                18 => 26,
                19 => 27,
                20 => 28,
                _ => 1,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###""(3)√""###,
            r###"")""###,
            r###""*""###,
            r###""**""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###""=""###,
            r###""^""###,
            r###""cbrt(""###,
            r###""cos(""###,
            r###""ln(""###,
            r###""log""###,
            r###""log(""###,
            r###""sin(""###,
            r###""sqrt(""###,
            r###""tan(""###,
            r###""|""###,
            r###""√""###,
            r###"r#"[-0-9]+\\.?[0-9]*"#"###,
            r###"r#"[_a-zA-Z][_a-zA-Z0-9]*"#"###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input, 's>
    where 
    {
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __phantom: core::marker::PhantomData<(&'input (), &'s ())>,
    }
    impl<'input, 's> __state_machine::ParserDefinition for __StateMachine<'input, 's>
    where 
    {
        type Location = usize;
        type Error = &'static str;
        type Token = Token<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = f64;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&(), &())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 22 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&(), &())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.symtab,
                self.input,
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&(), &())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
        's,
    >(
        __token: &Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token(2, _) if true => Some(0),
            Token(3, _) if true => Some(1),
            Token(4, _) if true => Some(2),
            Token(5, _) if true => Some(3),
            Token(6, _) if true => Some(4),
            Token(7, _) if true => Some(5),
            Token(8, _) if true => Some(6),
            Token(9, _) if true => Some(7),
            Token(10, _) if true => Some(8),
            Token(11, _) if true => Some(9),
            Token(12, _) if true => Some(10),
            Token(13, _) if true => Some(11),
            Token(14, _) if true => Some(12),
            Token(15, _) if true => Some(13),
            Token(16, _) if true => Some(14),
            Token(17, _) if true => Some(15),
            Token(18, _) if true => Some(16),
            Token(19, _) if true => Some(17),
            Token(20, _) if true => Some(18),
            Token(21, _) if true => Some(19),
            Token(0, _) if true => Some(20),
            Token(1, _) if true => Some(21),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
        's,
    >(
        __token_index: usize,
        __token: Token<'input>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(11, __tok0) | Token(12, __tok0) | Token(13, __tok0) | Token(14, __tok0) | Token(15, __tok0) | Token(16, __tok0) | Token(17, __tok0) | Token(18, __tok0) | Token(19, __tok0) | Token(20, __tok0) | Token(21, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct StatementParser {
        builder: __lalrpop_util::lexer::MatcherBuilder,
        _priv: (),
    }

    impl StatementParser {
        pub fn new() -> StatementParser {
            let __builder = super::__intern_token::new_builder();
            StatementParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            's,
        >(
            &self,
            symtab: &'s mut HashMap<String, f64>,
            input: &'input str,
        ) -> Result<f64, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            __state_machine::Parser::drive(
                __StateMachine {
                    symtab,
                    input,
                    __phantom: core::marker::PhantomData::<(&(), &())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> Option<Result<f64,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            1 => {
                __reduce1(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            2 => {
                __reduce2(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            3 => {
                __reduce3(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            4 => {
                __reduce4(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            5 => {
                __reduce5(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            6 => {
                __reduce6(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            7 => {
                __reduce7(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            8 => {
                __reduce8(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            9 => {
                __reduce9(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            10 => {
                __reduce10(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            11 => {
                __reduce11(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            12 => {
                __reduce12(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            13 => {
                __reduce13(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            14 => {
                __reduce14(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            15 => {
                __reduce15(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            16 => {
                __reduce16(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            17 => {
                __reduce17(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            18 => {
                __reduce18(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            19 => {
                __reduce19(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            20 => {
                __reduce20(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            21 => {
                __reduce21(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            22 => {
                __reduce22(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            23 => {
                __reduce23(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            24 => {
                __reduce24(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            25 => {
                __reduce25(symtab, input, __lookahead_start, __symbols, core::marker::PhantomData::<(&(), &())>)
            }
            26 => {
                // Term = Symbol => ActionFn(25);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action25::<>(symtab, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 5)
            }
            27 => {
                // Term = Term, Symbol => ActionFn(26);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action26::<>(symtab, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 5)
            }
            28 => {
                // __Statement = Statement => ActionFn(0);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(symtab, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, f64, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "+", Factor => ActionFn(3);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = Expr, "-", Factor => ActionFn(4);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action4::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "sin(", Expr, ")" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce3<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "cos(", Expr, ")" => ActionFn(6);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action6::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce4<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "tan(", Expr, ")" => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce5<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "√", Term => ActionFn(8);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action8::<>(symtab, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce6<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "sqrt(", Expr, ")" => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce7<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "(3)√", Term => ActionFn(10);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action10::<>(symtab, input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (2, 0)
    }
    pub(crate) fn __reduce8<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "cbrt(", Expr, ")" => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce9<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "log(", Expr, ")" => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce10<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "log", Term, "(", Expr, ")" => ActionFn(13);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant1(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action13::<>(symtab, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (5, 0)
    }
    pub(crate) fn __reduce11<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = "ln(", Expr, ")" => ActionFn(14);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce12<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Expr = Factor => ActionFn(15);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action15::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce13<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "^", Term => ActionFn(16);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce14<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "**", Term => ActionFn(17);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action17::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce15<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Term => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce16<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Term => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce17<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "(", Expr, ")" => ActionFn(20);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action20::<>(symtab, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 1)
    }
    pub(crate) fn __reduce18<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Factor = Term => ActionFn(21);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action21::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce19<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Num = r#"[-0-9]+\\.?[0-9]*"# => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce20<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Statement = Expr => ActionFn(1);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce21<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Statement = Symbol, "=", Expr => ActionFn(2);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action2::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce22<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Symbol = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(28);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action28::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce23<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Term = Num => ActionFn(22);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce24<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expr, ")" => ActionFn(23);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action23::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
    pub(crate) fn __reduce25<
        'input,
        's,
    >(
        symtab: &'s mut HashMap<String, f64>,
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut alloc::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: core::marker::PhantomData<(&'input (), &'s ())>,
    ) -> (usize, usize)
    {
        // Term = "|", Expr, "|" => ActionFn(24);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action24::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 5)
    }
}
pub use self::__parse__Statement::StatementParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use std::collections::HashMap;
    use lalrpop_util::ParseError;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    pub fn new_builder() -> __lalrpop_util::lexer::MatcherBuilder {
        let __strs: &[(&str, bool)] = &[
            ("^([\\-0-9]+\\.?[0-9]*)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^(\\()", false),
            ("^(\\(3\\)√)", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\*\\*)", false),
            ("^(\\+)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(=)", false),
            ("^(\\^)", false),
            ("^(cbrt\\()", false),
            ("^(cos\\()", false),
            ("^(ln\\()", false),
            ("^(log)", false),
            ("^(log\\()", false),
            ("^(sin\\()", false),
            ("^(sqrt\\()", false),
            ("^(tan\\()", false),
            ("^(\\|)", false),
            ("^(√)", false),
            (r"^(\s*)", true),
        ];
        __lalrpop_util::lexer::MatcherBuilder::new(__strs.iter().copied()).unwrap()
    }
}
pub(crate) use self::__lalrpop_util::lexer::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, s, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
) -> f64
{
    {
        symtab.insert(s,e);
        e
    }
}

#[allow(unused_variables)]
fn __action3<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, f64, usize),
) -> f64
{
    e+f
}

#[allow(unused_variables)]
fn __action4<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, f, _): (usize, f64, usize),
) -> f64
{
    e-f
}

#[allow(unused_variables)]
fn __action5<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.sin()
}

#[allow(unused_variables)]
fn __action6<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.cos()
}

#[allow(unused_variables)]
fn __action7<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.tan()
}

#[allow(unused_variables)]
fn __action8<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
) -> f64
{
    t.sqrt()
}

#[allow(unused_variables)]
fn __action9<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.sqrt()
}

#[allow(unused_variables)]
fn __action10<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
) -> f64
{
    t.cbrt()
}

#[allow(unused_variables)]
fn __action11<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.cbrt()
}

#[allow(unused_variables)]
fn __action12<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.log10()
}

#[allow(unused_variables)]
fn __action13<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.log(t)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.ln()
}

#[allow(unused_variables)]
fn __action15<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    __0
}

#[allow(unused_variables)]
fn __action16<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, f, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
) -> f64
{
    f.powf(t)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, f, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
) -> f64
{
    f.powf(t)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, f, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
) -> f64
{
    f*t
}

#[allow(unused_variables)]
fn __action19<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, f, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, f64, usize),
) -> f64
{
    f/t
}

#[allow(unused_variables)]
fn __action20<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, f, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    f*e
}

#[allow(unused_variables)]
fn __action21<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    __0
}

#[allow(unused_variables)]
fn __action22<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, __0, _): (usize, f64, usize),
) -> f64
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    __0
}

#[allow(unused_variables)]
fn __action24<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    e.abs()
}

#[allow(unused_variables)]
fn __action25<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, s, _): (usize, String, usize),
) -> Result<f64,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    match symtab.get(&s){
        Some(v)=>Ok(*v),
        None=>Err(ParseError::User{error:"Undefined Symbol"})       // x
    }
}

#[allow(unused_variables)]
fn __action26<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, t, _): (usize, f64, usize),
    (_, s, _): (usize, String, usize),
) -> Result<f64,__lalrpop_util::ParseError<usize,Token<'input>,&'static str>>
{
    match symtab.get(&s){
        Some(v)=>Ok(t*v),
        None=>Err(ParseError::User{error:"Undefined Symbol"})       // 5x
    }
}

#[allow(unused_variables)]
fn __action27<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> f64
{
    f64::from_str(s).unwrap()
}

#[allow(unused_variables)]
fn __action28<
    'input,
    's,
>(
    symtab: &'s mut HashMap<String, f64>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> String
{
    s.to_owned()
}

pub trait __ToTriple<'input, 's, >
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>;
}

impl<'input, 's, > __ToTriple<'input, 's, > for (usize, Token<'input>, usize)
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        Ok(value)
    }
}
impl<'input, 's, > __ToTriple<'input, 's, > for Result<(usize, Token<'input>, usize), &'static str>
{
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize), __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
