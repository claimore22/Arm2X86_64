## USMMLA

Unsigned by signed 8-bit integer matrix multiply-accumulate to 32-bit integer This instruction multiplies the 2×8 matrix of unsigned 8-bit integer values held in each 128-bit segment of the first source vector by the 8×2 matrix of signed 8-bit integer values in the corresponding segment of the second source vector. The resulting 2×2 widened 32-bit integer matrix product is then destructively added to the 32-bit integer matrix accumulator held in the corresponding segment of the addend and destination vector. This is equivalent to performing an 8-way dot product per destination element. This instruction is unpredicated. ID_AA64ZFR0_EL1.I8MM indicates whether this instruction is implemented. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) \|\| !IsFeatureImplemented(FEAT_I8MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let op1_unsigned : boolean = TRUE; let op2_unsigned : boolean = FALSE;

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL) = Zeros{}; var op1, op2 : bits(128); var res, addend : bits(128); for s = 0 to segments-1 do op1 = operand1[s\*:(128)]; op2 = operand2[s\*:(128)]; addend = operand3[s\*:(128)]; res = [MatMulAdd](shared_pseudocode.html#func_MatMulAdd_5)(addend, op1, op2, op1_unsigned, op2_unsigned); result[s\*:(128)] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
