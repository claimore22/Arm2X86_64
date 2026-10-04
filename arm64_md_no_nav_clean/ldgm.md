## LDGM

Load tag multiple This instruction reads a naturally aligned block of N Allocation Tags, where the size of N is identified in GMID_EL1.BS, and writes the Allocation Tag read from address A to the destination register at 4\*A<7:4>+3:4\*A<7:4>. Bits of the destination register not written with an Allocation Tag are set to 0. This instruction is UNDEFINED at EL0. This instruction generates an Unchecked access.

#### Decode

if !IsFeatureImplemented(FEAT_MTE2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn);

### Operation

if [PSTATE](shared_pseudocode.html#global_PSTATE).EL == [EL0](shared_pseudocode.html#global_EL0) then Undefined(); end; var data : bits(64) = Zeros{}; var address : bits(64); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let size : integer{} = 4 \* (2 ^ (UInt(GMID_EL1().BS))); address = AlignDownSize(address, size); let count : integer = size >> [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE); var index : integer = UInt(address[[LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE)+3:[LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE)]); let stzgm : boolean = FALSE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDGSTG](shared_pseudocode.html#func_CreateAccDescLDGSTG_3)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), stzgm, t); for i = 0 to count-1 do let tag : bits(4) = [AArch64_MemTag](shared_pseudocode.html#accessor_AArch64_MemTag_2)(address, accdesc); data[index\*:4] = tag; address = [AddressIncrement](shared_pseudocode.html#func_AddressIncrement_3)(address, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE), accdesc); index = index + 1; end; [X](shared_pseudocode.html#accessor_X_2){64}(t) = data;
