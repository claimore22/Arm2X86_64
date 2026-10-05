## FCVTPS (scalar SIMD&FP)

Floating-point convert to signed integer, rounding toward plus infinity (scalar SIMD&FP) This instruction converts the floating-point value in the SIMD&FP source register to a 32-bit or 64-bit signed integer using the Round toward Plus Infinity rounding mode, and writes the result to the SIMD&FP destination register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FPRCVT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let intsize : integer{} = 32 << UInt(sf); let fltsize : integer{} = 8 << UInt(ftype XOR '10');

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var fltval : bits(fltsize); var intval : bits(intsize); let merge : boolean = [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; fltval = [V](shared_pseudocode.html#accessor_V_2){fltsize}(n); intval = [FPToFixed](shared_pseudocode.html#func_FPToFixed_7){intsize, fltsize}(fltval, 0, FALSE, FPCR(), [FPRounding_POSINF](shared_pseudocode.html#enum_FPRounding_POSINF)); result[0+:intsize] = intval; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
