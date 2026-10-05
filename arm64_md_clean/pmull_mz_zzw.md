## PMULL

Multi-vector polynomial multiply long This instruction performs a polynomial multiplication over [0, 1] of the even-numbered elements of the first source vector by the corresponding elements of the second source vector, and places the results in the overlapping double-width elements of the first destination vector. The same operation is performed with odd-numbered elements of the source vectors, writing to the second destination vector. This instruction is unpredicated. This instruction is legal when executed in Streaming SVE mode if both FEAT_SSVE_AES and FEAT_SVE_AES2 are implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_AES2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd::'0');

### Operation

if IsFeatureImplemented(FEAT_SSVE_AES) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result_lo : bits(VL); var result_hi : bits(VL); for e = 0 to elements-1 do let element1_lo : bits(esize DIV 2) = operand1[(2\*e + 0)\*:(esize DIV 2)]; let element2_lo : bits(esize DIV 2) = operand2[(2\*e + 0)\*:(esize DIV 2)]; let element1_hi : bits(esize DIV 2) = operand1[(2\*e + 1)\*:(esize DIV 2)]; let element2_hi : bits(esize DIV 2) = operand2[(2\*e + 1)\*:(esize DIV 2)]; result_lo[e\*:esize] = [PolynomialMult](shared_pseudocode.html#func_PolynomialMult_4){esize DIV 2, esize DIV 2}(element1_lo, element2_lo); result_hi[e\*:esize] = [PolynomialMult](shared_pseudocode.html#func_PolynomialMult_4){esize DIV 2, esize DIV 2}(element1_hi, element2_hi); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d + 0) = result_lo; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d + 1) = result_hi;
