## NOT

Bitwise NOT (vector) This instruction reads each vector element from the source SIMD&FP register, places the inverse of each value into a vector, and writes the vector to the destination SIMD&FP register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. This instruction is used by the alias [MVN](mvn_not_advsimd.html "Bitwise NOT (vector)").

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8; let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV 8;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var result : bits(datasize); var element : bits(esize); for e = 0 to elements-1 do element = operand[e\*:esize]; result[e\*:esize] = NOT(element); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
