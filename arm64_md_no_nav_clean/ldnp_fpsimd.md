## LDNP (SIMD&FP)

Load pair of SIMD&FP registers, with non-temporal hint This instruction loads a pair of SIMD&FP registers from memory, issuing a hint to the memory system that the access is non-temporal. The address that is used for the load is calculated from a base register value and an optional immediate offset. For information about non-temporal pair instructions, see Load/Store SIMD and Floating-point non-temporal pair. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

// Empty.



For information about the CONSTRAINED UNPREDICTABLE behavior of this instruction, see Architectural Constraints on UNPREDICTABLE behaviors, and particularly LDNP (SIMD&FP).

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var address : bits(64); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, tagchecked, privileged, ispair); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(2\*datasize) = [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc); if rt_unknown then [V](shared_pseudocode.html#accessor_V_2){datasize}(t) = ARBITRARY : bits(datasize); elsif [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [V](shared_pseudocode.html#accessor_V_2){datasize}(t2) = data[(datasize-1):0]; [V](shared_pseudocode.html#accessor_V_2){datasize}(t) = data[(2\*datasize-1):datasize]; else [V](shared_pseudocode.html#accessor_V_2){datasize}(t) = data[(datasize-1):0]; [V](shared_pseudocode.html#accessor_V_2){datasize}(t2) = data[(2\*datasize-1):datasize]; end;
