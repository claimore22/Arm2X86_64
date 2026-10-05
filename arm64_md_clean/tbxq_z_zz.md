## TBXQ

Programmable table lookup within each quadword vector segment (merging) This instruction reads, for each 128-bit destination vector segment, each element of the corresponding second source (index) vector segment and uses its value to select an indexed element from the corresponding first source (table) vector segment. The indexed table element is placed in the element of the destination vector that corresponds to the index vector element. If an index value is greater than or equal to the number of elements in a 128-bit vector segment then the corresponding destination vector element is left unchanged. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let elements : integer = 128 DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(d); for s = 0 to segments-1 do for e = 0 to elements-1 do let idx : integer = UInt(operand2[(s \* elements + e)\*:esize]); if idx < elements then result[(s \* elements + e)\*:esize] = operand1[(s \* elements + idx)\*:esize]; end; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
