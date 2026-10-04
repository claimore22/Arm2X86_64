## BLR

Branch with link to register This instruction calls a subroutine at an address in a register, setting register X30 to PC+4. This instruction provides a hint that this is a subroutine call.

#### Decode

let n : integer{} = UInt(Rn);

### Operation

let target : bits(64) = X{}(n); if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then [AddGCSRecord](shared_pseudocode.html#func_AddGCSRecord_1)([PC64](shared_pseudocode.html#func_PC64_0)() + 4); end; // Value in BTypeNext will be used to set PSTATE.BTYPE [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '10'; [X](shared_pseudocode.html#accessor_X_2){64}(30) = [PC64](shared_pseudocode.html#func_PC64_0)() + 4; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_INDCALL](shared_pseudocode.html#enum_BranchType_INDCALL), branch_conditional);
