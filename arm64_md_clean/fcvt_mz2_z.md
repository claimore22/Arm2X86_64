## FCVT (widening)

Multi-vector half-precision convert to single-precision This instruction converts each element of the source vector from half-precision to single-precision floating-point, and places the results in the double-width destination elements of the destination vectors. This instruction follows SME2 floating-point numerical behaviors corresponding to instructions that place their results in one or more SVE Z vectors. This instruction is unpredicated. ID_AA64SMFR0_EL1.F16F16 indicates whether this instruction is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SME_F16F16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let d : integer = UInt(Zd::'0');

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let operand : bits(VL) = Z{}(n); var result : bits(2\*VL); for e = 0 to elements-1 do let element : bits(16) = operand[e\*:16]; let res : bits(32) = [FPConvertSVE](shared_pseudocode.html#func_FPConvertSVE_4){32, 16}(element, FPCR()); result[e\*:32] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+0) = result[VL-1:0]; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+1) = result[2\*VL-1:VL];
