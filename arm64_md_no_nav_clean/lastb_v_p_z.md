## LASTB (SIMD&FP scalar)

Extract last element to SIMD&FP scalar register This instruction extracts the Last active element to a SIMD&FP scalar register. If there is an Active element, the Last active element from the final source vector register is extracted. If there are no Active elements, the highest-numbered element is extracted. Then the extracted element is placed in the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Vd); let isBefore : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = Z{}(n); var last : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if isBefore then if last < 0 then last = elements - 1; end; else last = last + 1; if last >= elements then last = 0; end; end; [V](shared_pseudocode.html#accessor_V_2){esize}(d) = operand[last\*:esize];
