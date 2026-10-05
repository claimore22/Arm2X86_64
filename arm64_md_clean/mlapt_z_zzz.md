## MLAPT

Multiply-add checked pointer vectors This instruction multiplies, with overflow check, the elements of the first and second source vectors and adds the results of multiplication, with pointer check, to elements of the third source (addend) vector. The final results are destructively placed in the destination and third source (addend) vector. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) \|\| !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 64; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let element1 : integer = SInt(operand1[e\*:64]); let element2 : integer = SInt(operand2[e\*:64]); let product : integer = element1 \* element2; let overflow : boolean = (product != SInt(product[63:0])); let addend : bits(64) = operand3[e\*:64]; result[e\*:64] = [PointerMultiplyAddCheck](shared_pseudocode.html#func_PointerMultiplyAddCheck_3)(addend + product, addend, overflow); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
