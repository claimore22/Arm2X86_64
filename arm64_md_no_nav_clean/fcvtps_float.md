## FCVTPS (scalar)

Floating-point convert to signed integer, rounding toward plus infinity (scalar) This instruction converts the floating-point value in the SIMD&FP source register to a 32-bit or 64-bit signed integer using the Round towards Plus Infinity rounding mode, and writes the result to the general-purpose destination register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let intsize : integer{} = 32 << UInt(sf); let fltsize : integer{} = 8 << UInt(ftype XOR '10'); let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRounding_POSINF](shared_pseudocode.html#enum_FPRounding_POSINF); let unsigned : boolean = FALSE;

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let fltval : bits(fltsize) = V{}(n); let fracbits : integer = 0; [X](shared_pseudocode.html#accessor_X_2){intsize}(d) = [FPToFixed](shared_pseudocode.html#func_FPToFixed_7){intsize, fltsize}(fltval, fracbits, unsigned, FPCR(), rounding);
