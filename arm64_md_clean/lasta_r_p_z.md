## LASTA (scalar)

Extract element after last to general-purpose register This instruction extracts the element after the Last active element to a general-purpose register. If there is an Active element, the element after the Last active element modulo the number of elements from the final source vector register is extracted. If there are no Active elements, element zero is extracted. Then the extracted element is zero-extended and placed in the destination general-purpose register.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let rsize : integer{} = if esize < 64 then 32 else 64; let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Rd); let isBefore : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = Z{}(n); var result : bits(rsize); var last : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if isBefore then if last < 0 then last = elements - 1; end; else last = last + 1; if last >= elements then last = 0; end; end; result = ZeroExtend{rsize}(operand[last\*:esize]); [X](shared_pseudocode.html#accessor_X_2){rsize}(d) = result;
