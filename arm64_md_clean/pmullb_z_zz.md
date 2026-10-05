## PMULLB

Polynomial multiply long (bottom) This instruction performs a polynomial multiplication over [0, 1] of the even-numbered elements of the first source vector by the corresponding elements of the second source vector, and places the results in the overlapping double-width elements of the destination vector. This instruction is unpredicated. ID_AA64ZFR0_EL1.AES indicates whether the 128-bit element variant is implemented. The 128-bit element variant is legal when executed in Streaming SVE mode if one of the following is true: - Both FEAT_SSVE_AES and FEAT_SVE_PMULL128 are implemented. - FEAT_SME_FA64 is implemented and enabled. It has encodings from 2 classes: [16-bit or 64-bit elements](#iclass_16_bit_or_64_bit_elements) and [128-bit element](#iclass_128_bit_element)

#### Decode

if !IsFeatureImplemented(FEAT_SVE_PMULL128) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 128; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

if esize == 128 then if IsFeatureImplemented(FEAT_SSVE_AES) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; else [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize DIV 2) = operand1[(2\*e + 0)\*:(esize DIV 2)]; let element2 : bits(esize DIV 2) = operand2[(2\*e + 0)\*:(esize DIV 2)]; result[e\*:esize] = [PolynomialMult](shared_pseudocode.html#func_PolynomialMult_4){esize DIV 2, esize DIV 2}(element1, element2); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
