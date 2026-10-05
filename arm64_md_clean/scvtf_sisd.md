## SCVTF (scalar SIMD&FP)

Signed integer convert to floating-point (scalar SIMD&FP) This instruction converts the signed integer value in the SIMD&FP source register to a floating-point value using the rounding mode that is specified by the FPCR, and writes the result to the SIMD&FP destination register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FPRCVT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let intsize : integer{} = 32 << UInt(sf); let fltsize : integer{} = 8 << UInt(ftype XOR '10'); let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRoundingMode](shared_pseudocode.html#func_FPRoundingMode_1)(FPCR());

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var fltval : bits(fltsize); var intval : bits(intsize); let merge : boolean = [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()); var result : bits(128) = if merge then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; intval = [V](shared_pseudocode.html#accessor_V_2){intsize}(n); fltval = [FixedToFP](shared_pseudocode.html#func_FixedToFP_7){fltsize, intsize}(intval, 0, FALSE, FPCR(), rounding); result[0+:fltsize] = fltval; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
