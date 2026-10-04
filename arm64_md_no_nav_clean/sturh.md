## STURH

Store register halfword (unscaled) This instruction calculates an address from a base register value and an immediate offset, and stores a halfword to the calculated address, from a 32-bit register. For information about addressing modes, see Load/Store addressing modes.

#### Decode

let offset : bits(64) = SignExtend{}(imm9);

### Operation

var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc) = [X](shared_pseudocode.html#accessor_X_2){datasize}(t);
