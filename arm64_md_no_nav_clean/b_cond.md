## B.cond

Branch conditionally This instruction branches conditionally to a label at a PC-relative offset. This instruction provides a hint that this is not a subroutine call or return.

#### Decode

let offset : bits(64) = SignExtend{}(imm19::'00'); let condition : bits(4) = cond;

### Operation

let branch_conditional : boolean = TRUE; if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;
