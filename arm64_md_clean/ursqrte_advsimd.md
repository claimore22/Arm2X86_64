## URSQRTE

Unsigned reciprocal square root estimate This instruction reads each vector element from the source SIMD&FP register, calculates an approximate inverse square root for each value, places the result into a vector, and writes the vector to the destination SIMD&FP register. All the values in this instruction are unsigned integer values. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); if sz == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var result : bits(datasize); var element : bits(32); for e = 0 to elements-1 do element = operand[e\*:32]; result[e\*:32] = [UnsignedRSqrtEstimate](shared_pseudocode.html#func_UnsignedRSqrtEstimate_2){32}(element); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
