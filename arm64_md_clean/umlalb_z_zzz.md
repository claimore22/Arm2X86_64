## UMLALB (vectors)

Unsigned multiply-add long (bottom) This instruction multiplies the even-numbered unsigned elements of the first source vector by the corresponding elements of the second source vector, and destructively adds the results to the overlapping double-width elements of the addend vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(da); for e = 0 to elements-1 do let element1 : integer = UInt(operand1[(2\*e + 0)\*:(esize DIV 2)]); let element2 : integer = UInt(operand2[(2\*e + 0)\*:(esize DIV 2)]); let product = (element1 \* element2)[esize-1:0]; result[e\*:esize] = result[e\*:esize] + product; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
