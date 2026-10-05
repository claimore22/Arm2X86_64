## SSRA

Signed shift right and accumulate (immediate) This instruction shifts right by immediate each signed element of the source vector, preserving the sign bit, and destructively adds the truncated intermediate result to the corresponding elements of the addend vector. The immediate shift amount is an unsigned value in the range 1 to number of bits per element. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(4) = tszh::tszl; if tsize == '0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << HighestSetBitNZ(tsize); let n : integer = UInt(Zn); let da : integer = UInt(Zda); let shift : integer = (2 \* esize) - UInt(tsize::imm3);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let element : integer = SInt(operand1[e\*:esize]) >> shift; result[e\*:esize] = operand2[e\*:esize] + element[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
