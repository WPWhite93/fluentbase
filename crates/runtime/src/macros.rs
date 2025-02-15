#[macro_export]
macro_rules! forward_call_args {
    ($func:path, $caller:ident, []) => {
        $func($caller)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ty]) => {
        $func($caller, $a1)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty]) => {
        $func($caller, $a1, $a2)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty]) => {
        $func($caller, $a1, $a2, $a3)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty]) => {
        $func($caller, $a1, $a2, $a3, $a4)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty]) => {
        $func($caller, $a1, $a2, $a3, $a4, $a5)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty]) => {
        $func($caller, $a1, $a2, $a3, $a4, $a5, $a6)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty, $a7:ident :$t7:ty]) => {
        $func($caller, $a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty, $a7:ident :$t7:ty, $a8:ident :$t8:ty]) => {
        $func($caller, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8)
    };
    ($func:path, $caller:ident, [$a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty, $a7:ident :$t7:ty, $a8:ident :$t8:ty, $a9:ident :$t9:ty]) => {
        $func($caller, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9)
    };
}

#[macro_export]
macro_rules! count_call_args {
    () => {
        0
    };
    ($a1:ident :$t1:ty) => {
        1
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty) => {
        2
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty) => {
        3
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty) => {
        4
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty) => {
        5
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty) => {
        6
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty, $a7:ident :$t7:ty) => {
        7
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty, $a7:ident :$t7:ty, $a8:ident :$t8:ty) => {
        8
    };
    ($a1:ident :$t1:ident, $a2:ident :$t2:ty, $a3:ident :$t3:ty, $a4:ident :$t4:ty, $a5:ident :$t5:ty, $a6:ident :$t6:ty, $a7:ident :$t7:ty, $a8:ident :$t8:ty, $a9:ident :$t9:ty) => {
        9
    };
}

#[macro_export]
macro_rules! count_ret_args {
    (u32) => {
        1
    };
    (i32) => {
        1
    };
    (u64) => {
        1
    };
    (i64) => {
        1
    };
    (f32) => {
        1
    };
    (f64) => {
        1
    };
    (bool) => {
        1
    };
    ($out:ty) => {
        0
    };
}

#[macro_export]
macro_rules! impl_runtime_handler {
    ($runtime_handler:ty, $sys_func:ident, fn $module:ident::$name:ident($($t:tt)*) -> $out:tt) => {
        impl $crate::instruction::RuntimeHandler for $runtime_handler {
            const MODULE_NAME: &'static str = stringify!($module);
            const FUNC_NAME: &'static str = stringify!($name);

            const FUNC_INDEX: fluentbase_types::SysFuncIdx = fluentbase_types::SysFuncIdx::$sys_func;

            fn register_handler<DB: IJournaledTrie>(
                linker: &mut rwasm::Linker<RuntimeContext<DB>>,
                store: &mut rwasm::Store<RuntimeContext<DB>>,
            ) {
                use rwasm::AsContextMut;
                let func = rwasm::Func::wrap(
                    store.as_context_mut(),
                    |caller: Caller<'_, RuntimeContext<DB>>, $($t)*| -> Result<$out, rwasm::core::Trap> {
                        return $crate::forward_call_args! { Self::fn_handler, caller, [$($t)*] };
                    });
                let wrapped_index = store.inner.wrap_stored(rwasm::engine::bytecode::FuncIdx::from(Self::FUNC_INDEX as u32));
                linker.engine().register_trampoline(wrapped_index, func);
                linker.define(
                    stringify!($module),
                    stringify!($name),
                    func
                ).unwrap();
            }
        }
    };
}

#[macro_export]
macro_rules! forward_call {
    ($linker:tt, $store:tt, $module:literal, $name:literal, fn $func:ident($($t:tt)*) -> $out:ty) => {
        $linker.define(
            $module,
            $name,
            Func::wrap(
                $store.as_context_mut(),
                |caller: Caller<'_, RuntimeContext<DB>>, $($t)*| -> Result<$out, Trap> {
                    return forward_call_args! { $func, caller, [$($t)*] };
                })
        ).unwrap();
    };
}

#[cfg(test)]
mod tests {
    macro_rules! test_macro {
        ($val:ident -> $out:tt) => {
            const $val: usize = count_ret_args!($out);
        };
    }

    test_macro!(A -> u32);
    test_macro!(B -> ());
    test_macro!(C -> i32);
    test_macro!(D -> bool);

    #[test]
    fn test_count_ret_macro() {
        assert_eq!(A, 1);
        assert_eq!(B, 0);
        assert_eq!(C, 1);
        assert_eq!(D, 1);
    }
}
