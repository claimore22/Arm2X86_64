## EORBT

Interleaving exclusive-OR (bottom, top) This instruction performs an interleaving exclusive-OR between the even-numbered elements of the first source vector register and the odd-numbered elements of the second source vector register, placing the result in the even-numbered elements of the destination vector, leaving the odd-numbered elements unchanged. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let sel1 : integer = 0; let sel2 : integer = 1;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(d); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[(2\*e + sel1)\*:esize]; let element2 : bits(esize) = operand2[(2\*e + sel2)\*:esize]; result[(2\*e + sel1)\*:esize] = element1 XOR element2; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
