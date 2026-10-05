## SQSHLU

Signed saturating shift left unsigned by immediate This instruction shifts left by immediate each active signed element of the source vector, and destructively places the results in the corresponding elements of the source vector. Each result element is saturated to the N-bit element's unsigned integer range 0 to (2<sup>N</sup>)-1. The immediate shift amount is an unsigned value in the range 0 to number of bits per element minus 1. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(4) = tszh::tszl; if tsize == '0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << HighestSetBitNZ(tsize); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let shift : integer = UInt(tsize::imm3) - esize;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(dn); let mask : bits(PL) = P{}(g); var result : bits(VL); for e = 0 to elements-1 do let element1 : integer = SInt(operand1[e\*:esize]); if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let res : integer = element1 << shift; result[e\*:esize] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(res); else result[e\*:esize] = operand1[e\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
