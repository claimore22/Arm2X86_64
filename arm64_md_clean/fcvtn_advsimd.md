## FCVTN, FCVTN2 (double to single-precision, single to half-precision)

Floating-point convert to lower precision narrow (vector) This instruction reads each vector element in the SIMD&FP source register, converts each result to half the precision of the source element, writes the final result to a vector, and writes the vector to the lower or upper half of the destination SIMD&FP register. The destination vector elements are half as long as the source vector elements. The rounding mode is determined by the FPCR. FCVTN writes the vector to the lower half of the destination register and clears the upper half. FCVTN2 writes the vector to the upper half of the destination register without affecting the other bits of the register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let esize : integer{} = 16 << UInt(sz); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(2\*datasize) = V{}(n); var result : bits(datasize); for e = 0 to elements-1 do result[e\*:esize] = [FPConvert](shared_pseudocode.html#func_FPConvert_4){esize, 2\*esize}(operand[e\*:(2\*esize)], FPCR()); end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){datasize}(d, part) = result;
