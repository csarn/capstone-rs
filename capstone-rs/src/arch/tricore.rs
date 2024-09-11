//! Contains mips-specific types

use core::convert::From;
use core::{cmp, fmt, slice};

use capstone_sys::{cs_tricore, cs_tricore_op, tricore_op_mem, tricore_op_type};

// XXX todo(tmfink): create rusty versions
pub use capstone_sys::tricore_insn_group as TricoreInsnGroup;
pub use capstone_sys::tricore_insn as TricoreInsn;
pub use capstone_sys::tricore_reg as TricoreReg;

pub use crate::arch::arch_builder::tricore::*;
use crate::arch::DetailsArchInsn;
use crate::instruction::{RegId, RegIdInt};

/// Contains TRICORE-specific details for an instruction
pub struct TricoreInsnDetail<'a>(pub(crate) &'a cs_tricore);

impl_PartialEq_repr_fields!(TricoreInsnDetail<'a> [ 'a ];
    operands
);

/// TRICORE operand
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TricoreOperand {
    /// Register
    Reg(RegId),

    /// Immediate
    Imm(i32),

    /// Memory
    Mem(TricoreOpMem),

    /// Invalid
    Invalid,
}

impl Default for TricoreOperand {
    fn default() -> Self {
        TricoreOperand::Invalid
    }
}

/// TRICORE memory operand
#[derive(Debug, Copy, Clone)]
pub struct TricoreOpMem(pub(crate) tricore_op_mem);

impl TricoreOpMem {
    /// Base register
    pub fn base(&self) -> RegId {
        RegId(self.0.base as RegIdInt)
    }

    /// Disp value
    pub fn disp(&self) -> i32 {
        self.0.disp
    }
}

impl_PartialEq_repr_fields!(TricoreOpMem;
    base, disp
);

impl cmp::Eq for TricoreOpMem {}

impl<'a> From<&'a cs_tricore_op> for TricoreOperand {
    fn from(insn: &cs_tricore_op) -> TricoreOperand {
        match insn.type_ {
            tricore_op_type::TRICORE_OP_REG => {
                TricoreOperand::Reg(RegId(unsafe { insn.__bindgen_anon_1.reg } as RegIdInt))
            }
            tricore_op_type::TRICORE_OP_IMM => TricoreOperand::Imm(unsafe { insn.__bindgen_anon_1.imm }),
            tricore_op_type::TRICORE_OP_MEM => {
                TricoreOperand::Mem(TricoreOpMem(unsafe { insn.__bindgen_anon_1.mem }))
            }
            tricore_op_type::TRICORE_OP_INVALID => TricoreOperand::Invalid,
        }
    }
}

def_arch_details_struct!(
    InsnDetail = TricoreInsnDetail;
    Operand = TricoreOperand;
    OperandIterator = TricoreOperandIterator;
    OperandIteratorLife = TricoreOperandIterator<'a>;
    [ pub struct TricoreOperandIterator<'a>(slice::Iter<'a, cs_tricore_op>); ]
    cs_arch_op = cs_tricore_op;
    cs_arch = cs_tricore;
);

#[cfg(test)]
mod test {
    use super::*;
    use capstone_sys::*;

    #[test]
    fn test_tricore_op_from() {
        let op = cs_tricore_op {
            type_: tricore_op_type::TRICORE_OP_INVALID,
            __bindgen_anon_1: cs_tricore_op__bindgen_ty_1 { reg: 0 },
            access: 1
        };
        assert_eq!(TricoreOperand::from(&op), TricoreOperand::Invalid);
    }
}
