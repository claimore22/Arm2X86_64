## SUQADD

Signed saturating unsigned add This instruction adds active unsigned elements of the source vector to the corresponding signed elements of the addend vector, and destructively places the results in the corresponding elements of the addend vector. Each result element is saturated to the N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; let element2 : bits(esize) = operand2[e\*:esize]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then result[e\*:esize] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){esize}(SInt(element1) + UInt(element2)); else result[e\*:esize] = operand1[e\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
