## UMINV

Unsigned minimum reduction to scalar This instruction determines the unsigned minimum of the elements of the source vector, and places the result in the SIMD&FP scalar destination register. Inactive elements in the source vector are treated as the maximum unsigned integer for the element size.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Vd); let unsigned : boolean = TRUE;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var minimum : integer = if unsigned then (2^esize - 1) else (2^(esize-1) - 1); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let opelt : bits(esize) = operand[e\*:esize]; let element : integer = if unsigned then UInt(opelt) else SInt(opelt); minimum = Min(minimum, element); end; end; [V](shared_pseudocode.html#accessor_V_2){esize}(d) = minimum[esize-1:0];
