## SQDMULLT (indexed)

Signed saturating doubling multiply by indexed element (top) This instruction multiplies the odd-numbered signed elements within each 128-bit segment of the first source vector by the specified element in the corresponding second source vector segment, and then doubles and places the results in overlapping double-width elements of the destination vector register. Each result element is saturated to the double-width N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1. The elements within the second source vector are specified using an immediate index that selects the same element position within each 128-bit vector segment. The index range is from 0 to one less than the number of elements per 128-bit segment. It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let index : integer = UInt(i2h::i2l); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let sel : integer = 1;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); let eltspersegment : integer = 128 DIV (2 \* esize); let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let s : integer = e - (e MOD eltspersegment); let element1 : integer = SInt(operand1[(2 \* e + sel)\*:esize]); let element2 : integer = SInt(operand2[(2 \* s + index)\*:esize]); let res : integer = 2 \* element1 \* element2; result[e\*:(2\*esize)] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){2\*esize}(res); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
