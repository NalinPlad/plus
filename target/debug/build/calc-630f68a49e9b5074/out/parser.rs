// auto-generated: "lalrpop 0.19.8"
// sha3: 14344f12b0857a828f068d243b4b713c613a4e8dd5706696fd08b69d0dad143e
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
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 1
        -9, -9, -9, -9, -9, -9, -9, 0, -9, 0, 22,
        // State 2
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 3
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 4
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 5
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 6
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 7
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 8
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 9
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 10
        3, 0, 0, 0, 0, 0, 0, 0, 0, 21, 22,
        // State 11
        -7, -7, -7, -7, -7, -7, -7, 0, -7, 0, 22,
        // State 12
        -6, -6, -6, -6, -6, -6, -6, 0, -6, 0, 22,
        // State 13
        -8, -8, -8, -8, -8, -8, -8, 0, -8, 0, 22,
        // State 14
        -5, -5, -5, -5, -5, -5, -5, 0, -5, 0, 22,
        // State 15
        0, 0, 0, 0, 4, 5, 0, 0, 0, 0, 0,
        // State 16
        6, -4, 7, 8, -4, -4, 9, 0, 10, 0, 0,
        // State 17
        -14, -14, -14, -14, -14, -14, -14, 0, -14, 0, -14,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -16, 0, -16, -16, -16, -16, -16, 11, -16, 0, -16,
        // State 20
        -10, -10, -10, -10, -10, -10, -10, 0, -10, 0, -10,
        // State 21
        -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, -13,
        // State 22
        -17, -17, -17, -17, -17, -17, -17, 0, -17, 0, -17,
        // State 23
        0, 30, 0, 0, 4, 5, 0, 0, 0, 0, 0,
        // State 24
        -16, -16, -16, -16, -16, -16, -16, 0, -16, 0, -16,
        // State 25
        0, -1, 7, 8, -1, -1, 9, 0, 10, 0, 0,
        // State 26
        0, -2, 7, 8, -2, -2, 9, 0, 10, 0, 0,
        // State 27
        0, 31, 0, 0, 4, 5, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 4, 5, 0, 0, 0, 0, 0,
        // State 29
        -15, -15, -15, -15, -15, -15, -15, 0, -15, 0, -15,
        // State 30
        0, -3, 0, 0, -3, -3, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 11 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        0,
        // State 1
        -9,
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
        -7,
        // State 12
        -6,
        // State 13
        -8,
        // State 14
        -5,
        // State 15
        -11,
        // State 16
        -4,
        // State 17
        -14,
        // State 18
        -18,
        // State 19
        -16,
        // State 20
        -10,
        // State 21
        -13,
        // State 22
        -17,
        // State 23
        0,
        // State 24
        -16,
        // State 25
        -1,
        // State 26
        -2,
        // State 27
        0,
        // State 28
        -12,
        // State 29
        -15,
        // State 30
        -3,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => match state {
                2 => 23,
                5 => 27,
                10 => 28,
                _ => 15,
            },
            1 => match state {
                3 => 25,
                4 => 26,
                _ => 16,
            },
            2 => 17,
            3 => 18,
            4 => match state {
                0 => 19,
                2..=10 => 24,
                _ => 22,
            },
            5 => match state {
                6 => 11,
                7 => 12,
                8 => 13,
                9 => 14,
                _ => 1,
            },
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""**""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###""=""###,
            r###""^""###,
            r###"r#"[0-9]+\\.?[0-9]*"#"###,
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
            __action(state, 11 - 1)
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
            Token(0, _) if true => Some(9),
            Token(1, _) if true => Some(10),
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
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 => match __token {
                Token(2, __tok0) | Token(3, __tok0) | Token(4, __tok0) | Token(5, __tok0) | Token(6, __tok0) | Token(7, __tok0) | Token(8, __tok0) | Token(9, __tok0) | Token(10, __tok0) | Token(0, __tok0) | Token(1, __tok0) if true => __Symbol::Variant0(__tok0),
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
                // Term = Symbol => ActionFn(14);
                let __sym0 = __pop_Variant2(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = match super::__action14::<>(symtab, input, __sym0) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (1, 5)
            }
            16 => {
                // Term = Term, Symbol => ActionFn(15);
                assert!(__symbols.len() >= 2);
                let __sym1 = __pop_Variant2(__symbols);
                let __sym0 = __pop_Variant1(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = match super::__action15::<>(symtab, input, __sym0, __sym1) {
                    Ok(v) => v,
                    Err(e) => return Some(Err(e)),
                };
                __symbols.push((__start, __Symbol::Variant1(__nt), __end));
                (2, 5)
            }
            17 => {
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
        // Expr = Factor, "(", Expr, ")" => ActionFn(5);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action5::<>(symtab, input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (4, 0)
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
        // Expr = Factor => ActionFn(6);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action6::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 0)
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
        // Factor = Factor, "^", Term => ActionFn(7);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action7::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
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
        // Factor = Factor, "**", Term => ActionFn(8);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action8::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
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
        // Factor = Factor, "*", Term => ActionFn(9);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action9::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
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
        // Factor = Factor, "/", Term => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant1(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(symtab, input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (3, 1)
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
        // Factor = Term => ActionFn(11);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 1)
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
        // Num = r#"[0-9]+\\.?[0-9]*"# => ActionFn(16);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action16::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 2)
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
        // Statement = Expr => ActionFn(1);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 3)
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
        // Symbol = r#"[_a-zA-Z][_a-zA-Z0-9]*"# => ActionFn(17);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 4)
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
        // Term = Num => ActionFn(12);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<>(symtab, input, __sym0);
        __symbols.push((__start, __Symbol::Variant1(__nt), __end));
        (1, 5)
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
        // Term = "(", Expr, ")" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant1(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(symtab, input, __sym0, __sym1, __sym2);
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
            ("^([0-9]+\\.?[0-9]*)", false),
            ("^([A-Z_a-z][0-9A-Z_a-z]*)", false),
            ("^(\\()", false),
            ("^(\\))", false),
            ("^(\\*)", false),
            ("^(\\*\\*)", false),
            ("^(\\+)", false),
            ("^(\\-)", false),
            ("^(/)", false),
            ("^(=)", false),
            ("^(\\^)", false),
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
    (_, f, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, f64, usize),
    (_, _, _): (usize, &'input str, usize),
) -> f64
{
    f*e
}

#[allow(unused_variables)]
fn __action6<
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
fn __action7<
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
fn __action8<
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
fn __action9<
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
fn __action10<
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
fn __action11<
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
fn __action12<
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
fn __action13<
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
fn __action14<
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
fn __action15<
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
fn __action16<
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
fn __action17<
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
