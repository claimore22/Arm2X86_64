## TBZ

Test bit and branch if zero This instruction compares the value of a test bit with zero, and conditionally branches to a label at a PC-relative offset if the comparison is equal. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect condition flags.

#### Decode

let t : integer = UInt(Rt); let datasize : integer{} = 32 << UInt(b5); let bit_pos : integer = UInt(b5::b40); let offset : bits(64) = SignExtend{}(imm14::'00');

### Operation

let operand : bits(datasize) = X{}(t); let branch_conditional : boolean = TRUE; if operand[bit_pos] == '0' then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;
