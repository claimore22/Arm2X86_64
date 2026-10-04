## STG

Store Allocation Tag This instruction stores an Allocation Tag to memory. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag Granule. The Allocation Tag is calculated from the Logical Address Tag in the source register. This instruction generates an Unchecked access. It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Signed offset](#iclass_signed_offset)

#### Decode

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let offset : bits(64) = LSL(SignExtend{64}(imm9), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE)); let writeback : boolean = FALSE; let postindex : boolean = FALSE;

### Operation

var address : bits(64); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let stzgm : boolean = FALSE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDGSTG](shared_pseudocode.html#func_CreateAccDescLDGSTG_3)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), stzgm, t); if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; let data : bits(64) = if t == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(t); let tag : bits(4) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(data); [AArch64_MemTag](shared_pseudocode.html#accessor_AArch64_MemTag_2)(address, accdesc) = tag; if writeback then if postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;
