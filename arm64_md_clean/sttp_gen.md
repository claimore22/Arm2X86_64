## STTP

Store unprivileged pair of registers This instruction calculates an address from a base register value and an immediate offset, and stores two 64-bit doublewords to the calculated address, from two registers. Explicit Memory effects produced by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either: - The instruction is executed at EL1. - The instruction is executed at EL2 when the Effective value of HCR_EL2().{E2H, TGE} is '11'. Otherwise, the Explicit Memory effects operate with the restrictions determined by the Exception level at which the instruction is executed. For information about addressing modes, see Load/Store addressing modes. It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Signed offset](#iclass_signed_offset)

#### Decode

if !IsFeatureImplemented(FEAT_LSUI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let wback : boolean = FALSE; let postindex : boolean = FALSE;



STTP has the same CONSTRAINED UNPREDICTABLE behavior as STP. See Architectural Constraints on UNPREDICTABLE behaviors, and particularly STP.

### Operation

var address : bits(64); var data1 : bits(datasize); var data2 : bits(datasize); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)(); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_7)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, privileged, tagchecked, ispair, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if rt_unknown && t == n then data1 = ARBITRARY : bits(datasize); else data1 = [X](shared_pseudocode.html#accessor_X_2){datasize}(t); end; if rt_unknown && t2 == n then data2 = ARBITRARY : bits(datasize); else data2 = [X](shared_pseudocode.html#accessor_X_2){datasize}(t2); end; let data : bits(2\*datasize) = (if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then data1::data2 else data2::data1); [Mem](shared_pseudocode.html#accessor_Mem_3){2 \* datasize}(address, accdesc) = data; if wback then if postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;
