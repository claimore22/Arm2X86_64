## LDR (immediate)

Load register (immediate) This instruction loads a word or doubleword from memory and writes it to a register. The address that is used for the load is calculated from a base register and an immediate offset. For information about addressing modes, see Load/Store addressing modes. The Unsigned offset variant scales the immediate offset value by the size of the value accessed before adding it to the base register value. It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Unsigned offset](#iclass_unsigned_offset)

#### Decode

var wback : boolean = FALSE; let postindex : boolean = FALSE; let scale : integer{} = UInt(size); let offset : bits(64) = LSL(ZeroExtend{64}(imm12), scale);



For information about the CONSTRAINED UNPREDICTABLE behavior of this instruction, see Architectural Constraints on UNPREDICTABLE behaviors, and particularly LDR (immediate).

### Operation

var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; let data : bits(datasize) = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = ZeroExtend{regsize}(data); if wback then if wb_unknown then address = ARBITRARY : bits(64); elsif postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;
