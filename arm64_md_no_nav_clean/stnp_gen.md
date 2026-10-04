## STNP

Store pair of registers, with non-temporal hint This instruction calculates an address from a base register value and an immediate offset, and stores two 32-bit words or two 64-bit doublewords to the calculated address, from two registers. For information about addressing modes, see Load/Store addressing modes. For information about non-temporal pair instructions, see Load/Store non-temporal pair.

#### Decode

let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let nontemporal : boolean = TRUE; let scale : integer{} = 2 + UInt(opc[1]); let datasize : integer{} = 8 << scale; let offset : bits(64) = LSL(SignExtend{64}(imm7), scale); let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let dbytes : integer{} = datasize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_7)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, privileged, tagchecked, ispair, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(2\*datasize)= (if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [X](shared_pseudocode.html#accessor_X_2){datasize}(t) ::[X](shared_pseudocode.html#accessor_X_2){datasize}(t2) else [X](shared_pseudocode.html#accessor_X_2){datasize}(t2)::[X](shared_pseudocode.html#accessor_X_2){datasize}(t)); [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc) = data;
