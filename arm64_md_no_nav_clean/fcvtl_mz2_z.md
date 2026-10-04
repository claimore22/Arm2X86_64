## FCVTL

Multi-vector half-precision convert to deinterleaved single-precision This instruction converts each element of the source vector from half-precision to single-precision floating-point, and places the two-way deinterleaved results in the double-width destination elements of the destination vectors. This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors. This instruction is unpredicated. ID_AA64SMFR0_EL1.F16F16 indicates whether this instruction is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SME_F16F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd::'0');

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let pairs : integer = VL DIV 32; let operand : bits(VL) = Z{}(n); var result0 : bits(VL); var result1 : bits(VL); for p = 0 to pairs-1 do let element1 : bits(16) = operand[(2\*p+0)\*:16]; let element2 : bits(16) = operand[(2\*p+1)\*:16]; let res1 : bits(32) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){32, 16}(element1, FPCR()); let res2 : bits(32) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){32, 16}(element2, FPCR()); result0[p\*:32] = res1; result1[p\*:32] = res2; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result0; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result1;
