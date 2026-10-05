## BL

Branch with link This instruction branches to a PC-relative offset, setting register X30 to PC+4. This instruction provides a hint that this is a subroutine call.

#### Decode

let offset : bits(64) = SignExtend{}(imm26::'00'); let d : integer = 30;

### Operation

if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then [AddGCSRecord](shared_pseudocode.html#func_AddGCSRecord_1)([PC64](shared_pseudocode.html#func_PC64_0)() + 4); end; [X](shared_pseudocode.html#accessor_X_2){64}(d) = [PC64](shared_pseudocode.html#func_PC64_0)() + 4; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIRCALL](shared_pseudocode.html#enum_BranchType_DIRCALL), branch_conditional);
