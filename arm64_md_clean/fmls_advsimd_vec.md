## FMLS (vector)

Floating-point fused multiply-subtract from accumulator (vector) This instruction multiplies corresponding floating-point values in the vectors in the two source SIMD&FP registers, negates the product, adds the result to the corresponding vector element of the destination SIMD&FP register, and writes the result to the destination SIMD&FP register. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Half-precision](#iclass_half_precision) and [Single-precision and double-precision](#iclass_single_precision_and_double_precision)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 32 << UInt(sz); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); let operand3 : bits(datasize) = V{}(d); var result : bits(datasize); var element1 : bits(esize); var element2 : bits(esize); for e = 0 to elements-1 do element1 = [FPNeg](shared_pseudocode.html#func_FPNeg_3){esize}(operand1[e\*:esize], FPCR()); element2 = operand2[e\*:esize]; result[e\*:esize] = [FPMulAdd](shared_pseudocode.html#func_FPMulAdd_5){esize}(operand3[e\*:esize], element1, element2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
