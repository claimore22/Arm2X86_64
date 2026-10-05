## SSHLLB

Signed shift left long by immediate (bottom) This instruction shifts left by immediate each even-numbered signed element of the source vector, and places the results in the overlapping double-width elements of the destination vector. The immediate shift amount is an unsigned value in the range 0 to number of bits per element minus 1. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(3) = tszh::tszl; if tsize == '000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << HighestSetBitNZ(tsize); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let shift : integer = UInt(tsize::imm3) - esize;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); let operand : bits(VL) = Z{}(n); var result : bits(VL); for e = 0 to elements-1 do let element : bits(esize) = operand[(2\*e + 0)\*:esize]; let shifted_value : integer = SInt(element) << shift; result[e\*:(2\*esize)] = shifted_value[2\*esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
