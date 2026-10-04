## LDSETB, LDSETAB, LDSETALB, LDSETLB

Atomic bit set on byte This instruction atomically loads an 8-bit byte from memory, performs a bitwise OR with the value held in a register on it, and stores the result back to memory. The value initially loaded from memory is returned in the destination register. - If the destination register is not WZR, LDSETAB and LDSETALB load from memory with acquire semantics. - LDSETLB and LDSETALB store to memory with release semantics. - LDSETB has neither acquire nor release semantics. For more information about memory ordering semantics, see Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes. This instruction is used by the alias [STSETB, STSETLB](stsetb_ldsetb.html "Atomic bit set on byte, without return").

#### Decode

if !IsFeatureImplemented(FEAT_LSE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let s : integer{} = UInt(Rs); let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquire : boolean = A == '1' && Rt != '11111'; let release : boolean = R == '1'; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAtomicOp](shared_pseudocode.html#func_CreateAccDescAtomicOp_7)([MemAtomicOp_ORR](shared_pseudocode.html#enum_MemAtomicOp_ORR), acquire, release, tagchecked, privileged, t, s); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let comparevalue : bits(8) = ARBITRARY : bits(8); // Irrelevant when not executing CAS let value : bits(8) = [X](shared_pseudocode.html#accessor_X_2){8}(s); let data : bits(8) = MemAtomic{}(address, comparevalue, value, accdesc); if t != 31 then [X](shared_pseudocode.html#accessor_X_2){32}(t) = ZeroExtend{32}(data); end;
