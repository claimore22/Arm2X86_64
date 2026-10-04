## FMAXNMV

Floating-point maximum number across vector This instruction compares all the vector elements in the source SIMD&FP register, and writes the largest of the values as a scalar to the destination SIMD&FP register. All the values in this instruction are floating-point values. Regardless of the value of FPCR.AH, the behavior is as follows: - Negative zero compares less than positive zero. - If one value is numeric and the other is a quiet NaN, the result is the numeric value. - When FPCR.DN is 0, if either value is a signaling NaN or if both values are NaNs, the result is a quiet NaN. - When FPCR.DN is 1, if either value is a signaling NaN or if both values are NaNs, the result is Default NaN. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision](#iclass_single_precision)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz::Q != '01' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; // .4S only let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 32; let datasize : integer{} = 64 << UInt(Q);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); [V](shared_pseudocode.html#accessor_V_2){esize}(d) = [FPReduce](shared_pseudocode.html#func_FPReduce_5){esize, datasize}([ReduceOp_FMAXNUM](shared_pseudocode.html#enum_ReduceOp_FMAXNUM), operand, FPCR());
