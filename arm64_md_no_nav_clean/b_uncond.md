## B

Branch This instruction branches unconditionally to a label at a PC-relative offset. This instruction provides a hint that this is not a subroutine call or return.

#### Decode

let offset : bits(64) = SignExtend{}(imm26::'00');

### Operation

let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional);
