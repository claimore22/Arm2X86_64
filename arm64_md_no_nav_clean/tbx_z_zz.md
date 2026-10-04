## TBX

Programmable table lookup in single vector table (merging) This instruction reads each element of the second source (index) vector and uses its value to select an indexed element from a table of elements in the first source vector, and places the indexed element in the destination vector element corresponding to the index vector element. If an index value is greater than or equal to the number of vector elements, then the corresponding destination vector element is left unchanged. Since the index values can select any element in a vector this operation is not naturally vector length agnostic.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(d); for e = 0 to elements-1 do let element2 : integer = UInt(operand2[e\*:esize]); if element2 < elements then result[e\*:esize] = operand1[element2\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
