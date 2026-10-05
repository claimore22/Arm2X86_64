## LDRSW (literal)

Load register signed word (literal) This instruction calculates an address from the PC value and an immediate offset, loads a word from memory, and writes it to a register. For information about addressing modes, see Load/Store addressing modes.

#### Decode

let t : integer{} = UInt(Rt); let nontemporal : boolean = FALSE; let tagchecked : boolean = FALSE; let offset : bits(64) = SignExtend{}(imm19::'00');

### Operation

let address : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() + offset; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); let data : bits(32) = [Mem](shared_pseudocode.html#accessor_Mem_3){32}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){64}(t) = SignExtend{64}(data);
