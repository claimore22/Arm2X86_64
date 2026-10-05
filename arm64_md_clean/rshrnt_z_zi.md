## RSHRNT

Rounding shift right narrow by immediate (top) This instruction shifts each unsigned integer value in the source vector elements right by an immediate value, and places the rounded results in the odd-numbered half-width destination elements, leaving the even-numbered elements unchanged. The immediate shift amount is an unsigned value in the range 1 to number of bits per element. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(3) = tszh::tszl; if tsize == '000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << HighestSetBitNZ(tsize); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let shift : integer = (2 \* esize) - UInt(tsize::imm3);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); let operand : bits(VL) = Z{}(n); var result : bits(VL) = Z{}(d); for e = 0 to elements-1 do let element : bits(2\*esize) = operand[e\*:(2\*esize)]; let res : integer = (UInt(element) + (1 << (shift-1))) >> shift; result[(2\*e + 1)\*:esize] = res[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
