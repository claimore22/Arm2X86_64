## SLI

Shift left and insert (immediate) This instruction shifts each source vector element left by an immediate value, and inserts the result into the corresponding vector element in the destination vector register, merging the shifted bits from each source element with existing bits in each destination vector element. The immediate shift amount is an unsigned value in the range 0 to number of bits per element minus 1. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(4) = tszh::tszl; if tsize == '0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << HighestSetBitNZ(tsize); let n : integer = UInt(Zn); let d : integer = UInt(Zd); let shift : integer = UInt(tsize::imm3) - esize;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand : bits(VL) = Z{}(n); var result : bits(VL) = Z{}(d); for e = 0 to elements-1 do let element1 : bits(esize) = result[e\*:esize]; let element2 : bits(esize) = operand[e\*:esize]; let mask : bits(esize) = LSL(Ones{esize}, shift); let shiftedval : bits(esize) = LSL(element2, shift); result[e\*:esize] = (element1 AND (NOT mask)) OR shiftedval; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
