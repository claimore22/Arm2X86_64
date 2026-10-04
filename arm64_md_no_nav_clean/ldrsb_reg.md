## LDRSB (register)

Load register signed byte (register) This instruction calculates an address from a base register value and an offset register value, loads a byte from memory, sign-extends it, and writes it to a register. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if option[1] == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; // sub-word index let extend_type : [ExtendType](shared_pseudocode.html#type_ExtendType) = [DecodeRegExtend](shared_pseudocode.html#func_DecodeRegExtend_1)(option); let shift : integer{} = 0;

### Operation

let offset : bits(64) = ExtendReg{}(m, extend_type, shift); var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(8) = [Mem](shared_pseudocode.html#accessor_Mem_3){8}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = SignExtend{regsize}(data);
