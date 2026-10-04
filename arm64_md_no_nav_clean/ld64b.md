## LD64B

Single-copy atomic 64-byte Load This instruction derives an address from a base register value, loads eight 64-bit doublewords from a memory location, and writes them to consecutive registers. The load starts at register Xt, with the data being read as X(t+7)::X(t+6)::X(t+5)::X(t+4)::X(t+3)::X(t+2)::X(t+1)::Xt = Data[511:0]. The data is loaded atomically and is required to be 64-byte aligned. It is IMPLEMENTATION DEFINED which memory locations support this instruction. A memory location that supports LD64B also supports ST64B. For more information, including about the memory types accessible and how the accesses are performed, see Single-copy atomic 64-byte load/store.

#### Decode

if !IsFeatureImplemented(FEAT_LS64) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Rt[4:3] == '11' \|\| Rt[0] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let withstatus : boolean = FALSE; let t : integer{} = UInt(Rt); let n : integer{} = UInt(Rn); let tagchecked : boolean = n != 31;



For information about the CONSTRAINED UNPREDICTABLE behavior of this instruction, see Architectural Constraints on UNPREDICTABLE behaviors, and particularly LD64B.

### Operation

[CheckLDST64BEnabled](shared_pseudocode.html#func_CheckLDST64BEnabled_0)(); var data : bits(512); var address : bits(64); var value : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLS64](shared_pseudocode.html#func_CreateAccDescLS64_3)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), withstatus, tagchecked); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; data = [MemLoad64B](shared_pseudocode.html#func_MemLoad64B_2)(address, accdesc); for i = 0 to 7 do value = data[63+64\*i : 64\*i]; if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then value = [BigEndianReverse](shared_pseudocode.html#func_BigEndianReverse_2){64}(value); end; [X](shared_pseudocode.html#accessor_X_2){64}(t+i) = value; end;
