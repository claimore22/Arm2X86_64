## FCVTL, FCVTL2

Floating-point convert to higher precision long (vector) This instruction reads each element in a vector in the SIMD&FP source register, converts each value to double the precision of the source element using the rounding mode that is determined by the FPCR, and writes each result to the equivalent element of the vector in the SIMD&FP destination register. Where the operation lengthens a 64-bit vector to a 128-bit vector, the FCVTL2 variant operates on the elements in the top 64 bits of the source register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let esize : integer{} = 16 << UInt(sz); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = Vpart{}(n, part); var result : bits(2\*datasize); for e = 0 to elements-1 do result[e\*:(2\*esize)] = [FPConvert](shared_pseudocode.html#func_FPConvert_4){2\*esize, esize}(operand[e\*:esize], FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d) = result;
