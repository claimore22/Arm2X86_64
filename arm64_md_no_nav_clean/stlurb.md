## STLURB

Store-release register byte (unscaled) This instruction calculates an address from a base register value and an immediate offset, and stores a byte to the calculated address, from a 32-bit register. The instruction has memory ordering semantics as described in Load-Acquire, Load-AcquirePC, and Store-Release For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LRCPC2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let offset : bits(64) = SignExtend{}(imm9); let n : integer = UInt(Rn); let t : integer = UInt(Rt); let datasize : integer{} = 8; let acquire : boolean = FALSE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAcqRel](shared_pseudocode.html#func_CreateAccDescAcqRel_4)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), tagchecked, acquire, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc) = [X](shared_pseudocode.html#accessor_X_2){datasize}(t);
