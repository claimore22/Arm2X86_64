## LDTRSW

Load register signed word (unprivileged) This instruction loads a word from memory, sign-extends it to 64 bits, and writes the result to a register. The address that is used for the load is calculated from a base register and an immediate offset. Explicit Memory effects produced by the instruction behave as if the instruction was executed at EL0 if the Effective value of PSTATE.UAO is 0 and either: - The instruction is executed at EL1. - The instruction is executed at EL2 when the Effective value of HCR_EL2().{E2H, TGE} is '11'. Otherwise, the Explicit Memory effects operate with the restrictions determined by the Exception level at which the instruction is executed. For information about addressing modes, see Load/Store addressing modes.

#### Decode

let offset : bits(64) = SignExtend{}(imm9);

### Operation

var address : bits(64); let privileged : boolean = [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)(); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(datasize) = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){64}(t) = SignExtend{64}(data);
