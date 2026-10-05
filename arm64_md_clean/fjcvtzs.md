## FJCVTZS

Floating-point JavaScript convert to signed fixed-point, rounding toward zero This instruction converts the double-precision floating-point value in the SIMD&FP source register to a 32-bit signed integer using the Round towards Zero rounding mode, and writes the result to the general-purpose destination register. If the result is too large to be represented as a signed 32-bit integer, then the result is the integer modulo 2<sup>32</sup>, as held in a 32-bit signed integer. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_JSCVT) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let fltval : bits(64) = V{}(n); var intval : bits(32); var z : bit; (intval, z) = [FPToFixedJS](shared_pseudocode.html#func_FPToFixedJS_2)(fltval, FPCR()); [X](shared_pseudocode.html#accessor_X_2){32}(d) = intval; PSTATE.[N,Z,C,V] = '0'::z::'00';
