## LDPSW

Load pair of registers signed word This instruction calculates an address from a base register value and an immediate offset, loads two 32-bit words from memory, sign-extends them, and writes them to two registers. For information about addressing modes, see Load/Store addressing modes. It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Signed offset](#iclass_signed_offset)

#### Decode

var wback : boolean = FALSE; let postindex : boolean = FALSE;



For information about the CONSTRAINED UNPREDICTABLE behavior of this instruction, see Architectural Constraints on UNPREDICTABLE behaviors, and particularly LDPSW.

### Operation

var address : bits(64); var data1 : bits(datasize); var data2 : bits(datasize); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_7)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, ispair, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; let data : bits(2\*datasize) = [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc); if rt_unknown then data1 = ARBITRARY : bits(datasize); data2 = ARBITRARY : bits(datasize); elsif [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then data2 = data[(datasize-1):0]; data1 = data[(2\*datasize-1):datasize]; else data1 = data[(datasize-1):0]; data2 = data[(2\*datasize-1):datasize]; end; [X](shared_pseudocode.html#accessor_X_2){64}(t) = SignExtend{64}(data1); [X](shared_pseudocode.html#accessor_X_2){64}(t2) = SignExtend{64}(data2); if wback then if wb_unknown then address = ARBITRARY : bits(64); elsif postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;
