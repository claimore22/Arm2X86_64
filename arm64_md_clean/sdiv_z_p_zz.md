## SDIV

Signed divide (predicated) This instruction performs a signed divide on active elements of the first source vector by the corresponding elements of the second source vector and destructively places the quotient in the corresponding elements of the first source vector. Inactive elements in the destination vector register remain unmodified.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size IN {'0x'} then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let g : integer = UInt(Pg); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(m) else Zeros{VL}; var result : bits(VL); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then let dividend : integer = SInt(operand1[e\*:esize]); let divisor : integer = SInt(operand2[e\*:esize]); var quotient : integer; if divisor == 0 then quotient = 0; elsif (dividend < 0) == (divisor < 0) then quotient = Abs(dividend) DIVRM Abs(divisor); // same signs - positive result else quotient = -(Abs(dividend) DIVRM Abs(divisor)); // different signs - negative result end; result[e\*:esize] = quotient[esize-1:0]; else result[e\*:esize] = operand1[e\*:esize]; end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;
