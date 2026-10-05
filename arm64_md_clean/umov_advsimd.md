## UMOV

Unsigned move vector element to general-purpose register This instruction reads the unsigned integer from the source SIMD&FP register, zero-extends it to form a 32-bit or 64-bit value, and writes the result to the destination general-purpose register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This instruction is used by the alias [MOV (to general)](mov_umov_advsimd.html "Move vector element to general-purpose register").

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5[3:0]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << size; let datasize : integer{} = 32 << UInt(Q); if datasize == 64 && esize < 64 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if datasize == 32 && esize >= 64 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let index : integer = UInt(imm5[4:size+1]); let idxdsize : integer{} = 64 << UInt(imm5[4]);

### Operation

if index == 0 then [AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); else [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); end; let operand : bits(idxdsize) = V{}(n); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = ZeroExtend{datasize}(operand[index\*:esize]);
