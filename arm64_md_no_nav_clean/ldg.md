## LDG

Load Allocation Tag This instruction loads an Allocation Tag from a memory address, generates a Logical Address Tag from the Allocation Tag and merges it into the destination register. The address used for the load is calculated from the base register and an immediate signed offset scaled by the Tag Granule.

#### Decode

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let offset : bits(64) = LSL(SignExtend{64}(imm9), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE));

### Operation

var address : bits(64); var tag : bits(4); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let stzgm : boolean = FALSE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDGSTG](shared_pseudocode.html#func_CreateAccDescLDGSTG_3)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), stzgm, t); address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); address = AlignDownSize(address, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)); tag = [AArch64_MemTag](shared_pseudocode.html#accessor_AArch64_MemTag_2)(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){64}(t) = [AArch64_AddressWithAllocationTag](shared_pseudocode.html#func_AArch64_AddressWithAllocationTag_2)([X](shared_pseudocode.html#accessor_X_2){64}(t), tag);
