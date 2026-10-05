## STILP

Store-release ordered pair of registers This instruction calculates an address from a base register value and an optional offset, and stores two 32-bit words or two 64-bit doublewords to the calculated address, from two registers. For information on single-copy atomicity and alignment requirements, see Requirements for single-copy atomicity and Alignment of data accesses. The instruction also has memory ordering semantics, as described in Load-Acquire, Load-AcquirePC, and Store-Release, with the additional requirement that: - When using the pre-index addressing mode, the Memory effects associated with Xt2/Wt2 are Ordered-before the Memory effects associated with Xt1/Wt1. - For all other addressing modes, the Memory effects associated with Xt1/Wt1 are Ordered-before the Memory effects associated with Xt2/Wt2. For information about addressing modes, see Load/Store addressing modes.

#### Decode

if !IsFeatureImplemented(FEAT_LRCPC3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let ispair : boolean = TRUE; let wback : boolean = opc2[0] == '0';



STILP has the same CONSTRAINED UNPREDICTABLE behavior as STP. For information about this CONSTRAINED UNPREDICTABLE behavior, see Architectural Constraints on UNPREDICTABLE behaviors, and particularly STP and STILP.

### Operation

var address : bits(64); var data1 : bits(datasize); var data2 : bits(datasize); let dbytes : integer{} = datasize DIV 8; var accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescAcqRel](shared_pseudocode.html#func_CreateAccDescAcqRel_6)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), tagchecked, ispair, acqrel, t, t2); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); if rt_unknown && t == n then data1 = ARBITRARY : bits(datasize); else data1 = [X](shared_pseudocode.html#accessor_X_2){datasize}(t); end; if rt_unknown && t2 == n then data2 = ARBITRARY : bits(datasize); else data2 = [X](shared_pseudocode.html#accessor_X_2){datasize}(t2); end; var full_data : bits(2\*datasize); if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then full_data = data1::data2; else full_data = data2::data1; end; accdesc.highestaddressfirst = offset < 0; [Mem](shared_pseudocode.html#accessor_Mem_3){2\*datasize}(address, accdesc) = full_data; if wback then if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;
