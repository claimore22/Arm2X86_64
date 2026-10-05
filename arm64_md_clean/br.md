## BR

Branch to register This instruction branches unconditionally to an address in a register. This instruction provides a hint that this is not a subroutine call or return.

#### Decode

let n : integer{} = UInt(Rn);

### Operation

let target : bits(64) = X{}(n); // Value in BTypeNext will be used to set PSTATE.BTYPE if [InGuardedPage](shared_pseudocode.html#global_InGuardedPage) then if n == 16 \|\| n == 17 then [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '01'; else [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '11'; end; else [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '01'; end; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_INDIR](shared_pseudocode.html#enum_BranchType_INDIR), branch_conditional);
