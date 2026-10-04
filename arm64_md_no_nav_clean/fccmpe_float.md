## FCCMPE

Floating-point conditional signaling compare (scalar) This instruction compares the two SIMD&FP source register values and writes the result to the PSTATE.{N, Z, C, V} flags. If the condition does not pass, then the PSTATE.{N, Z, C, V} flags are set to the flag bit specifier. This instruction raises an Invalid Operation floating-point exception if either or both of the operands is any type of NaN. This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see Floating-point exceptions and exception traps. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 8 << UInt(ftype XOR '10'); let condition : bits(4) = cond; var flags : bits(4) = nzcv;

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then let signal_all_nans : boolean = TRUE; flags = [FPCompare](shared_pseudocode.html#func_FPCompare_5){datasize}(operand1, operand2, signal_all_nans, FPCR()); end; PSTATE.[N,Z,C,V] = flags;
