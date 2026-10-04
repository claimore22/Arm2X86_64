## STLRH

Store-release register halfword This instruction stores a halfword from a 32-bit register to a memory location. The instruction also has memory ordering semantics as described in Load-Acquire, Store-Release. For information about addressing modes, see Load/Store addressing modes.

#### Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let acquire : boolean = FALSE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAcqRel](shared_pseudocode.html#func_CreateAccDescAcqRel_4)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), tagchecked, acquire, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; [Mem](shared_pseudocode.html#accessor_Mem_3){16}(address, accdesc) = [X](shared_pseudocode.html#accessor_X_2){16}(t);
