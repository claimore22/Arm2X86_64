## STZGM

Store Allocation Tag and zero multiple This instruction writes a naturally aligned block of N Allocation Tags and stores zero to the associated data locations, where the size of N is identified in DCZID_EL0.BS, and the Allocation Tag is taken from the source register bits<3:0>. This instruction is UNDEFINED at EL0. This instruction generates an Unchecked access.

#### Decode

if !IsFeatureImplemented(FEAT_MTE2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn);

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; let data : bits(64) = X{}(t); let tag : bits(4) = data[3:0]; var address : bits(64); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let size : integer{} = 4 \* (2 ^ (UInt(DCZID_EL0().BS))); address = AlignDownSize(address, size); let count : integer = size >> [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE); let stzgm : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDGSTG](shared_pseudocode.html#func_CreateAccDescLDGSTG_3)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), stzgm, t); for i = 0 to count-1 do [AArch64_MemTag](shared_pseudocode.html#accessor_AArch64_MemTag_2)(address, accdesc) = tag; [Mem](shared_pseudocode.html#accessor_Mem_3){8\*[TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)}(address, accdesc) = Zeros{8\*[TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)}; address = [AddressIncrement](shared_pseudocode.html#func_AddressIncrement_3)(address, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE), accdesc); end;
