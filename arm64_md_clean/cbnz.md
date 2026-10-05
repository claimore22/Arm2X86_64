## CBNZ

Compare and branch on nonzero This instruction compares the value in a register with zero, and conditionally branches to a label at a PC-relative offset if the comparison is not equal. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.

#### Decode

let t : integer = UInt(Rt); let datasize : integer{} = 32 << UInt(sf); let offset : bits(64) = SignExtend{}(imm19::'00');

### Operation

let branch_conditional : boolean = TRUE; let operand1 : bits(datasize) = X{}(t); if !IsZero(operand1) then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;
