## LDXR

Load exclusive register This instruction derives an address from a base register value, loads a 32-bit word or a 64-bit doubleword from memory, and writes it to a register. The memory access is atomic. The PE marks the physical address being accessed as an exclusive access. This exclusive access mark is checked by Store Exclusive instructions. See Synchronization and semaphores. For information about addressing modes, see Load/Store addressing modes.

#### Decode

let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let elsize : integer{} = 8 << UInt(size); let regsize : integer{} = if elsize == 64 then 64 else 32; let acqrel : boolean = FALSE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); var data : bits(elsize); let dbytes : integer{} = elsize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescExLDST](shared_pseudocode.html#func_CreateAccDescExLDST_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), acqrel, tagchecked, privileged, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; [AArch64_SetExclusiveMonitors](shared_pseudocode.html#func_AArch64_SetExclusiveMonitors_3)(address, dbytes, accdesc); data = [Mem](shared_pseudocode.html#accessor_Mem_3){elsize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = ZeroExtend{regsize}(data);
