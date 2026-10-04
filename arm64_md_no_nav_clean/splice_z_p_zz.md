## SPLICE

Splice two vectors under predicate control This instruction selects a region from the first source vector and copies it to the lowest-numbered elements of the result. Any remaining elements of the result are set to a copy of the lowest-numbered elements from the second source vector. The region is selected using the first and last true elements in the vector select predicate register. The result is destructively placed in the destination and first source vector, or constructively placed in the destination vector. It has encodings from 2 classes: [Constructive](#iclass_constructive) and [Destructive](#iclass_destructive)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let v : integer = UInt(Pv); let dst : integer = UInt(Zdn); let s1 : integer = dst; let s2 : integer = UInt(Zm);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(v); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(s1) else Zeros{VL}; let operand2 : bits(VL) = Z{}(s2); var result : bits(VL); var x : integer = 0; var active : boolean = FALSE; let lastnum : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if lastnum >= 0 then for e = 0 to lastnum do active = active \|\| [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize); if active then result[x\*:esize] = operand1[e\*:esize]; x = x + 1; end; end; end; let nelements : integer = (elements - x) - 1; for e = 0 to nelements do result[x\*:esize] = operand2[e\*:esize]; x = x + 1; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst) = result;
