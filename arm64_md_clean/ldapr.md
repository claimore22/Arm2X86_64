## LDAPR

Load-acquire RCpc register This instruction derives an address from a base register value, loads a 32-bit word or 64-bit doubleword from the derived address in memory, and writes it to a register. If the destination register is not one of WZR or XZR, LDAPR loads from memory with AcquirePC semantics. For more information about memory ordering semantics, see Load-Acquire, Load-AcquirePC, and Store-Release. For information about addressing modes, see Load/Store addressing modes. It has encodings from 2 classes: [Post-index](#iclass_post_index) and [No offset](#iclass_no_offset)

#### Decode

if !IsFeatureImplemented(FEAT_LRCPC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let wback : boolean = FALSE; let offset : integer = 0; let wb_unknown : boolean = FALSE; let elsize : integer{} = 8 << UInt(size); let regsize : integer{} = if elsize == 64 then 64 else 32; let datasize : integer{} = elsize; let acquirepc : boolean = TRUE; let tagchecked : boolean = n != 31;

### Operation

var address : bits(64); var data : bits(datasize); let dbytes : integer{} = datasize DIV 8; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDAcqPC](shared_pseudocode.html#func_CreateAccDescLDAcqPC_3)(tagchecked, acquirepc, t); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; data = [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){regsize}(t) = ZeroExtend{regsize}(data); if wback then if wb_unknown then address = ARBITRARY : bits(64); else address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;
