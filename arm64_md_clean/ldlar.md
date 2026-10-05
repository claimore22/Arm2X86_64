## LDLAR

Load LOAcquire register This instruction loads a 32-bit word or 64-bit doubleword from memory, and writes it to a register. If the destination register is not one of WZR or XZR, LDLAR loads from memory with Acquire semantics. For more information about memory ordering semantics, see Load LOAcquire, Store LORelease and Load-Acquire, Load-AcquirePC, and Store-Release. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LOR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let elsize : integer{} = 8 << UInt(size); let regsize : integer{} = if elsize == 64 then 64 else 32; let acquire : boolean = TRUE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let dbytes : integer{} = elsize DIV 8; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLOR](shared_pseudocode.html#func_CreateAccDescLOR_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), tagchecked, acquire, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let data : bits(elsize) = [Mem](shared_pseudocode.html#accessor_Mem_3){elsize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = ZeroExtend{regsize}(data);
