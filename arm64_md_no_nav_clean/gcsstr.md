## GCSSTR

Guarded Control Stack store register This instruction stores a doubleword from a register to memory. The address that is used for the store is calculated from a base register.

#### Decode

if !IsFeatureImplemented(FEAT_GCS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn);

### Operation

var address : bits(64); let effective_el : bits(2) = [PSTATE](shared_pseudocode.html#global_PSTATE).EL; if effective_el == [PSTATE](shared_pseudocode.html#global_PSTATE).EL then [CheckGCSSTREnabled](shared_pseudocode.html#func_CheckGCSSTREnabled_0)(); end; let privileged : boolean = effective_el != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGCS](shared_pseudocode.html#func_CreateAccDescGCS_2)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), privileged); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; [Mem](shared_pseudocode.html#accessor_Mem_3){64}(address, accdesc) = [X](shared_pseudocode.html#accessor_X_2){64}(t);
