## STNP (SIMD&FP)

Store pair of SIMD&FP registers, with non-temporal hint This instruction stores a pair of SIMD&FP registers to memory, issuing a hint to the memory system that the access is non-temporal. The address used for the store is calculated from an address from a base register value and an immediate offset. For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point non-temporal pair. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

// Empty.

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var address : bits(64); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_5)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, tagchecked, privileged, ispair); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); var data : bits(2\*datasize); if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then data = [V](shared_pseudocode.html#accessor_V_2){datasize}(t) :: [V](shared_pseudocode.html#accessor_V_2){datasize}(t2); else data = [V](shared_pseudocode.html#accessor_V_2){datasize}(t2) :: [V](shared_pseudocode.html#accessor_V_2){datasize}(t); end; [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc) = data;
