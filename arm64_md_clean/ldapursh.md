## LDAPURSH

Load-acquire RCpc register signed halfword (unscaled) This instruction calculates an address from a base register and an immediate offset, loads a signed halfword from memory, sign-extends it, and writes it to a register. If the destination register is not one of WZR or XZR, LDAPURSH loads from memory with AcquirePC semantics. For more information about memory ordering semantics, see Load-Acquire, Load-AcquirePC, and Store-Release. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LRCPC2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let offset : bits(64) = SignExtend{}(imm9);

### Operation

var address : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDAcqPC](shared_pseudocode.html#func_CreateAccDescLDAcqPC_3)(tagchecked, acquirepc, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); let data : bits(datasize) = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = SignExtend{regsize}(data);
