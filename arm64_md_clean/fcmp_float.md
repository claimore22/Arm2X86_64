## FCMP

Floating-point quiet compare (scalar) This instruction compares the two SIMD&FP source register values, or the first SIMD&FP source register value and zero. It writes the result to the PSTATE.{N, Z, C, V} flags. This instruction raises an Invalid Operation floating-point exception if either or both of the operands is a signaling NaN. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt(Rm); // ignored when opc<0> == '1' let datasize : integer{} = 8 << UInt(ftype XOR '10'); let signal_all_nans : boolean = FALSE; let cmp_with_zero : boolean = (opc[0] == '1');

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = if cmp_with_zero then [FPZero](shared_pseudocode.html#func_FPZero_2){datasize}('0') else [V](shared_pseudocode.html#accessor_V_2){datasize}(m); PSTATE.[N,Z,C,V] = [FPCompare](shared_pseudocode.html#func_FPCompare_5){datasize}(operand1, operand2, signal_all_nans, FPCR());
