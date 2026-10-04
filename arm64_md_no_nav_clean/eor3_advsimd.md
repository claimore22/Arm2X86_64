## EOR3

Three-way exclusive-OR This instruction performs a three-way exclusive-OR of the values in the three source SIMD&FP registers, and writes the result to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SHA3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(m); let operand2 : bits(128) = V{}(n); let operand3 : bits(128) = V{}(a); [V](shared_pseudocode.html#accessor_V_2){128}(d) = operand2 XOR operand1 XOR operand3;
