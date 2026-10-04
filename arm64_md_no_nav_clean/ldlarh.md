## LDLARH

Load LOAcquire register halfword This instruction loads a halfword from memory, zero-extends it, and writes it to a register. If the destination register is not one of WZR or XZR, LDLARH loads from memory with Acquire semantics. For more information about memory ordering semantics, see Load LOAcquire, Store LORelease and Load-Acquire, Load-AcquirePC, and Store-Release. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LOR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquire : boolean = TRUE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLOR](shared_pseudocode.html#func_CreateAccDescLOR_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), tagchecked, acquire, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let data : bits(16) = [Mem](shared_pseudocode.html#accessor_Mem_3){16}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){32}(t) = ZeroExtend{32}(data);
