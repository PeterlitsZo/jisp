//! The builders for [Asm] and [IFunc].

use std::mem::swap;

use super::{Asm, IFunc, Stat};

/// The [Asm] builder.
pub(super) struct AsmBuilder {
    asm: Asm,
}

impl AsmBuilder {
    /// Create a new [AsmBuilder].
    pub(super) fn new() -> Self {
        Self { asm: Asm::new() }
    }

    /// Push an [IFunc] by call the given `ifunc_getter` argument.
    pub(super) fn push_ifunc_by<T>(&mut self, ifunc_getter: T) -> &mut Self
    where
        T: FnOnce(IFuncBuilder) -> IFunc,
    {
        self.asm.ifuncs.push(ifunc_getter(IFunc::builder()));
        self
    }

    /// Push an [IFunc].
    pub(super) fn push_ifunc(&mut self, ifunc: IFunc) -> &mut Self {
        self.asm.ifuncs.push(ifunc);
        self
    }

    /// Build the [Asm] and reset the builder itself.
    pub(super) fn build(&mut self) -> Asm {
        let mut tmp_asm = Asm::new();
        swap(&mut tmp_asm, &mut self.asm);
        tmp_asm
    }
}

/// The [IFunc] builder.
pub(super) struct IFuncBuilder {
    ifunc: IFunc,
}

impl IFuncBuilder {
    /// Create a new [IFuncBuilder].
    pub(super) fn new() -> Self {
        Self {
            ifunc: IFunc::new(),
        }
    }

    /// Set the locals' number.
    pub(super) fn set_locals(&mut self, locals: usize) -> &mut Self {
        self.ifunc.locals = locals;
        self
    }

    /// Push a [Stat].
    pub(super) fn push_stat(&mut self, stat: Stat) -> &mut Self {
        self.ifunc.stats.push(stat);
        self
    }

    /// Build the [IFunc] and reset the builder itself.
    pub(super) fn build(&mut self) -> IFunc {
        let mut tmp_ifunc = IFunc::new();
        swap(&mut tmp_ifunc, &mut self.ifunc);
        tmp_ifunc
    }
}
