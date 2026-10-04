## CLASTA (scalar)

Conditionally extract element after last to general-purpose register This instruction extracts and zero-extends the element after the Last active element, or if the Last active element is the final element it extracts element zero, from the source vector register, and destructively places the zero-extended value in the destination and first source general-purpose register. If there are no Active elements, the least significant element-size bits of the destination and first source general-purpose register are destructively zero-extended.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Rdn); let m : integer = UInt(Zm); let csize : integer{} = if esize < 64 then 32 else 64; let isBefore : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(esize) = X{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(csize); var last : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if last < 0 then result = ZeroExtend{csize}(operand1); else if !isBefore then last = last + 1; if last >= elements then last = 0; end; end; result = ZeroExtend{csize}(operand2[last\*:esize]); end; [X](shared_pseudocode.html#accessor_X_2){csize}(dn) = result;
