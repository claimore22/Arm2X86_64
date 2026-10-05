## FMINP (scalar)

Floating-point minimum of pair of elements (scalar) This instruction compares two vector elements in the source SIMD&FP register and writes the smallest of the floating-point values as a scalar to the destination SIMD&FP register. When FPCR.AH is 0, the behavior is as follows for each pairwise operation: - Negative zero compares less than positive zero. - When FPCR.DN is 0, if either element is a NaN, the result is a quiet NaN. - When FPCR.DN is 1, if either element is a NaN, the result is Default NaN. When FPCR.AH is 1, the behavior is as follows for each pairwise operation: - If both elements are zeros, regardless of the sign of either zero, the result is the second element. - If either element is a NaN, regardless of the value of FPCR.DN, the result is the second element. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 32 << UInt(sz); let datasize : integer{} = esize \* 2;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); let element1 : bits(esize) = operand[0\*:esize]; let element2 : bits(esize) = operand[1\*:esize]; [V](shared_pseudocode.html#accessor_V_2){esize}(d) = [FPMin](shared_pseudocode.html#func_FPMin_4){esize}(element1, element2, FPCR());
