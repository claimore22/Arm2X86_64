## FRECPX

Floating-point reciprocal exponent (scalar) This instruction finds an approximate reciprocal exponent for the source SIMD&FP register and writes the result to the destination SIMD&FP register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let esize : integer{} = 32 << UInt(sz);

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand : bits(esize) = V{}(n); let merge : boolean = [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; result[0+:esize] = [FPRecpX](shared_pseudocode.html#func_FPRecpX_3){esize}(operand, FPCR()); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
