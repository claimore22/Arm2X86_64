## FSCALE

Floating-point adjust exponent (predicated) This instruction multiplies the active floating-point elements of the first source vector by 2.0 to the power of the signed integer values in the corresponding elements of the second source vector and destructively places the results in the corresponding elements of the first source vector. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:(esize)]; if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let element2 : integer = SInt(operand2[e\*:(esize)]); result[e\*:(esize)] = [FPScale](shared_pseudocode.html#func_FPScale_4){esize}(element1, element2, FPCR()); else result[e\*:(esize)] = element1; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
