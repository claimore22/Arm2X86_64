## SQRDMLAH (indexed)

Signed saturating rounding doubling multiply-add high by indexed element This instruction multiplies all signed elements within each 128-bit segment of the first source vector by the specified signed element of the corresponding second source vector segment, and then doubles the result and destructively adds the rounded high half of each result to the corresponding elements of the addend and destination vector. Each destination element is saturated to the N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1. The elements within the second source vector are specified using an immediate index that selects the same element position within each 128-bit vector segment. The index range is from 0 to one less than the number of elements per 128-bit segment. It has encodings from 3 classes: [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let index : integer = UInt(i1); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let eltspersegment : integer = 128 DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = segmentbase + index; let element1 : integer = SInt(operand1[e\*:esize]); let element2 : integer = SInt(operand2[s\*:esize]); let element3 : integer = SInt(operand3[e\*:esize]); let res : integer = (element3 << esize) + (2 \* element1 \* element2); result[e\*:esize] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){esize}((res + (1 << (esize - 1))) >> esize); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
