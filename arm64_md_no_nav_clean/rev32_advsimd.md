## REV32 (vector)

Reverse elements in 32-bit words (vector) This instruction reverses the order of 8-bit or 16-bit elements in each word of the vector in the source SIMD&FP register, places the results into a vector, and writes the vector to the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let csize : integer{} = 64 >> UInt(o0::U); let esize : integer{} = 8 << UInt(size); if csize <= esize then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let datasize : integer{} = 64 << UInt(Q); let containers : integer = datasize DIV csize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var result : bits(datasize); for c = 0 to containers-1 do let container : bits(csize) = operand[c\*:csize]; result[c\*:csize] = [Reverse](shared_pseudocode.html#func_Reverse_3){csize}(container, esize); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
