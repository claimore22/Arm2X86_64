## CLASTA (SIMD&FP scalar)

Conditionally extract element after last to SIMD&FP scalar register This instruction extracts and zero-extends the element after the Last active element, or if the Last active element is the final element it extracts element zero, from the source vector register, and destructively places the zero-extended value in the destination and first source SIMD & floating-point scalar register. If there are no Active elements, the least significant element-size bits of the destination and first source SIMD & floating-point scalar register are destructively zero-extended.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Vdn); let m : integer = UInt(Zm); let isBefore : boolean = FALSE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(esize) = V{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(esize); var last : integer = [LastActiveElement](shared_pseudocode.html#func_LastActiveElement_3){PL}(mask, esize); if last < 0 then result = ZeroExtend{esize}(operand1); else if !isBefore then last = last + 1; if last >= elements then last = 0; end; end; result = operand2[last\*:esize]; end; [V](shared_pseudocode.html#accessor_V_2){esize}(dn) = result;
