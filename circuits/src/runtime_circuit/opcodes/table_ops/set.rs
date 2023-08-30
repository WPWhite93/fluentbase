use crate::{
    bail_illegal_opcode,
    constraint_builder::AdviceColumn,
    runtime_circuit::{
        constraint_builder::OpConstraintBuilder,
        execution_state::ExecutionState,
        opcodes::{ExecutionGadget, GadgetError, TraceStep},
    },
    util::Field,
};
use fluentbase_rwasm::engine::bytecode::Instruction;
use halo2_proofs::circuit::Region;
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub(crate) struct TableSetGadget<F: Field> {
    table_index: AdviceColumn,
    elem_index: AdviceColumn,
    elem_type: AdviceColumn,
    value: AdviceColumn,
    size: AdviceColumn,
    _pd: PhantomData<F>,
}

impl<F: Field> ExecutionGadget<F> for TableSetGadget<F> {
    const NAME: &'static str = "WASM_TABLE_SET";

    const EXECUTION_STATE: ExecutionState = ExecutionState::WASM_TABLE_SET;

    fn configure(cb: &mut OpConstraintBuilder<F>) -> Self {
        let table_index = cb.query_rwasm_value();
        let elem_index = cb.query_rwasm_value();
        let elem_type = cb.query_rwasm_value();
        let value = cb.query_rwasm_value();
        let size = cb.query_rwasm_value();
        cb.table_size(table_index.expr(), size.expr());
        cb.table_set(table_index.expr(), elem_index.expr(), value.expr());
        cb.stack_pop(elem_type.current());
        cb.stack_pop(elem_index.current());
        cb.stack_pop(value.current());
        cb.range_check_1024(elem_index.expr());
        cb.range_check_1024(size.expr() - elem_index.expr());
        Self {
            table_index,
            elem_index,
            elem_type,
            value,
            size,
            _pd: Default::default(),
        }
    }

    fn assign_exec_step(
        &self,
        region: &mut Region<'_, F>,
        offset: usize,
        trace: &TraceStep,
    ) -> Result<(), GadgetError> {
        let (table_index, elem_type, elem_index, value) = match trace.instr() {
            Instruction::TableFill(ti) =>
                ( ti,
                  trace.curr_nth_stack_value(0)?,
                  trace.curr_nth_stack_value(1)?,
                  trace.curr_nth_stack_value(2)?,
                ),
            _ => bail_illegal_opcode!(trace),
        };
        self.table_index.assign(region, offset, F::from(table_index.to_u32() as u64));
        self.elem_type.assign(region, offset, F::from(elem_type.to_bits()));
        self.elem_index.assign(region, offset, F::from(elem_index.to_bits()));
        self.value.assign(region, offset, F::from(value.to_bits()));
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::runtime_circuit::testing::test_ok_with_demo_table;
    use fluentbase_rwasm::instruction_set;

    #[test]
    fn table_set() {
        test_ok_with_demo_table(instruction_set! {
            I32Const(0)
            I32Const(0)
            RefFunc(0)
            TableSet(0)
            Drop
        });
    }
}
