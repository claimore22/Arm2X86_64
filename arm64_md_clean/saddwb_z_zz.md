## SADDWB

Signed add wide (bottom) This instruction adds the even-numbered signed elements of the second source vector to the overlapping double-width elements of the first source vector and places the results in the corresponding double-width elements of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : integer = SInt(operand1[e\*:esize]); let element2 : integer = SInt(operand2[(2\*e + 0)\*:(esize DIV 2)]); result[e\*:esize] = (element1 + element2)[esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
