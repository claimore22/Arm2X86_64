## FTSSEL

Floating-point trigonometric select coefficient This instruction selects either the element of the first source vector or the value 1.0, based on bit<0> of the corresponding element of the second source element, negates the result if bit<1> of the corresponding element of the second source element is set, and places the results in the destination vector. This instruction is unpredicated. FTSSEL can be combined with FTSMUL and FTMAD to calculate values for sin(k) and cos(k). For more information, see FTSMUL. The use of the second operand is consistent with it holding an integer corresponding to the desired sine-wave quadrant. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1[e\*:esize]; let element2 : bits(esize) = operand2[e\*:esize]; result[e\*:esize] = [FPTrigSSel](shared_pseudocode.html#func_FPTrigSSel_3){esize}(element1, element2); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
