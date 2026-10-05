## SQDMLSLT (vectors)

Signed saturating doubling multiply-subtract (top) This instruction multiplies the odd-numbered signed elements of the first source vector by the corresponding elements of the second source vector, and then doubles the result. Each intermediate value is saturated to the double-width N-bit value's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1 and is then destructively subtracted from the overlapping double-width elements of the addend and destination vector. Each destination element is saturated to the double-width N-bit element's signed integer range -2<sup>(N-1)</sup> to (2<sup>(N-1)</sup>)-1. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda); let sel1 : integer = 1; let sel2 : integer = 1;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL) = Z{}(da); for e = 0 to elements-1 do let element1 : integer = SInt(operand1[(2 \* e + sel1)\*:(esize DIV 2)]); let element2 : integer = SInt(operand2[(2 \* e + sel2)\*:(esize DIV 2)]); let element3 : integer = SInt(result[e\*:esize]); let product : integer = SInt([SignedSat](shared_pseudocode.html#func_SignedSat_2){esize}(2 \* element1 \* element2)); result[e\*:esize] = [SignedSat](shared_pseudocode.html#func_SignedSat_2){esize}(element3 - product); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
