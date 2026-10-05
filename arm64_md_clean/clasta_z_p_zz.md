## CLASTA (vectors)

Conditionally extract element after last to vector register This instruction extracts the element after the Last active element, or if the Last active element is the final element it extracts element zero, from the second source vector register, and then replicates the extracted element to destructively fill the destination and first source vector. If there are no Active elements, the destination and source vector are unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm); let isBefore : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); var last : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if last < 0 then result = operand1; else if !isBefore then last = last + 1; if last >= elements then last = 0; end; end; for e = 0 to elements-1 do result[e\*:esize] = operand2[last\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
