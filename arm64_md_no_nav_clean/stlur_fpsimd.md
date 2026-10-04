## STLUR (SIMD&FP)

Store-release SIMD&FP register (unscaled offset) This instruction stores a single SIMD&FP register to memory. The address that is used for the store is calculated from a base register value and an optional immediate offset. The instruction has memory ordering semantics, as described in Load-Acquire, Load-AcquirePC, and Store-Release. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) \|\| !IsFeatureImplemented(FEAT_LRCPC3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if opc[1] == '1' && size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let scale : integer{} = if opc[1] == '1' then 4 else UInt(size); let offset : bits(64) = SignExtend{}(imm9);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var address : bits(64); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMDAcqRel](shared_pseudocode.html#func_CreateAccDescASIMDAcqRel_2)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), tagchecked); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); [Mem](shared_pseudocode.html#accessor_Mem_3){datasize}(address, accdesc) = [V](shared_pseudocode.html#accessor_V_2){datasize}(t);
