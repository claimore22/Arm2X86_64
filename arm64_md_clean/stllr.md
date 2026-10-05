## STLLR

Store LORelease register This instruction stores a 32-bit word or a 64-bit doubleword to a memory location, from a register. The instruction also has memory ordering semantics as described in Load LOAcquire, Store LORelease. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LOR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let elsize : integer{} = 8 << UInt(size); let acquire : boolean = FALSE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let dbytes : integer{} = elsize DIV 8; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLOR](shared_pseudocode.html#func_CreateAccDescLOR_4)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), tagchecked, acquire, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; [Mem](shared_pseudocode.html#accessor_Mem_3){elsize}(address, accdesc) = [X](shared_pseudocode.html#accessor_X_2){elsize}(t);
