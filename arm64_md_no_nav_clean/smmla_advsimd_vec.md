## SMMLA (vector)

Signed 8-bit integer matrix multiply-accumulate to 32-bit integer (vector) This instruction multiplies the 2x8 matrix of signed 8-bit integer values in the first source vector by the 8x2 matrix of signed 8-bit integer values in the second source vector. The resulting 2x2 32-bit integer matrix product is destructively added to the 32-bit integer matrix accumulator in the destination vector. This is equivalent to performing an 8-way dot product per destination element. From Armv8.2 to Armv8.5, this is an OPTIONAL instruction. From Armv8.6 it is mandatory for implementations that include Advanced SIMD to support it. ID_AA64ISAR1_EL1.I8MM indicates whether this instruction is supported.

#### Decode

if !IsFeatureImplemented(FEAT_I8MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let op1_unsigned : boolean = FALSE; let op2_unsigned : boolean = FALSE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); let addend : bits(128) = V{}(d); [V](shared_pseudocode.html#accessor_V_2){128}(d) = [MatMulAdd](shared_pseudocode.html#func_MatMulAdd_5)(addend, operand1, operand2, op1_unsigned, op2_unsigned);
