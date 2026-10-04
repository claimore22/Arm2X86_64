## LDAPRH

Load-acquire RCpc register halfword This instruction derives an address from a base register value, loads a halfword from the derived address in memory, zero-extends it and writes it to a register. If the destination register is not one of WZR or XZR, LDAPRH loads from memory with AcquirePC semantics. For more information about memory ordering semantics, see Load-Acquire, Load-AcquirePC, and Store-Release. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LRCPC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquirepc : boolean = TRUE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); var data : bits(16); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDAcqPC](shared_pseudocode.html#func_CreateAccDescLDAcqPC_3)(tagchecked, acquirepc, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; data = [Mem](shared_pseudocode.html#accessor_Mem_3){16}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){32}(t) = ZeroExtend{32}(data);
