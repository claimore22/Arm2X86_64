## SWPH, SWPAH, SWPALH, SWPLH

Swap halfword in memory This instruction atomically loads a 16-bit halfword from a memory location, and stores the value held in a register back to the same memory location. The value initially loaded from memory is returned in the destination register. - If the destination register is not WZR, SWPAH and SWPALH load from memory with acquire semantics. - SWPLH and SWPALH store to memory with release semantics. - SWPH has neither acquire nor release semantics. For more information about memory ordering semantics, see Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LSE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquire : boolean = A == '1' && Rt != '11111'; let release : boolean = R == '1'; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); var data : bits(16); var store_value : bits(16); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAtomicOp](shared_pseudocode.html#func_CreateAccDescAtomicOp_7)([MemAtomicOp_SWP](shared_pseudocode.html#enum_MemAtomicOp_SWP), acquire, release, tagchecked, privileged, t, s); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; store_value = [X](shared_pseudocode.html#accessor_X_2){16}(s); let comparevalue : bits(16) = ARBITRARY : bits(16); // Irrelevant when not executing CAS data = [MemAtomic](shared_pseudocode.html#func_MemAtomic_5){16}(address, comparevalue, store_value, accdesc); [X](shared_pseudocode.html#accessor_X_2){32}(t) = ZeroExtend{32}(data);
