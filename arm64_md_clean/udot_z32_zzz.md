## UDOT (2-way, vectors)

Unsigned integer dot product (two-way) This instruction calculates the dot product of a pair of unsigned 16-bit integer values held in each 32-bit element of the first source vector multiplied by a pair of unsigned 16-bit integer values in the corresponding 32-bit element of the second source vector, and then destructively adds the widened dot product to the corresponding 32-bit element of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do var res : bits(esize) = operand3[e\*:esize]; for i = 0 to 1 do let element1 : integer = UInt(operand1[(2 \* e + i)\*:(esize DIV 2)]); let element2 : integer = UInt(operand2[(2 \* e + i)\*:(esize DIV 2)]); res = res + element1 \* element2; end; result[e\*:esize] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
