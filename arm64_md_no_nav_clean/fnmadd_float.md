## FNMADD

Floating-point negated fused multiply-add (scalar) This instruction multiplies the values of the first two SIMD&FP source registers, negates the product, subtracts the value of the third SIMD&FP source register, and writes the result to the destination SIMD&FP register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra); let esize : integer{} = 8 << UInt(ftype XOR '10');

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let addend : bits(esize) = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}([V](shared_pseudocode.html#accessor_V_2){esize}(a), FPCR()); let operand1 : bits(esize) = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}([V](shared_pseudocode.html#accessor_V_2){esize}(n), FPCR()); let operand2 : bits(esize) = V{}(m); var result : bits(128) = if [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()) then [V](shared_pseudocode.html#accessor_V_2){128}(a) else Zeros{128}; result[0+:esize] = [FPMulAdd](shared_pseudocode.html#func_FPMulAdd_5){esize}(addend, operand1, operand2, FPCR()); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;
