## RET

Return from subroutine This instruction branches unconditionally to an address in a register. This instruction provides a hint that this is a subroutine return.

#### Decode

let n : integer{} = UInt(Rn);

### Operation

var target : bits(64) = X{}(n); if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then target = [LoadCheckGCSRecord](shared_pseudocode.html#func_LoadCheckGCSRecord_2)(target, [GCSInstType_PRET](shared_pseudocode.html#enum_GCSInstType_PRET)); [SetCurrentGCSPointer](shared_pseudocode.html#func_SetCurrentGCSPointer_1)([GetCurrentGCSPointer](shared_pseudocode.html#func_GetCurrentGCSPointer_0)() + 8); end; // Value in BTypeNext will be used to set PSTATE.BTYPE [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '00'; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_RET](shared_pseudocode.html#enum_BranchType_RET), branch_conditional);
