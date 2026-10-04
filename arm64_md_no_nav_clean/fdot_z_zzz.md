## FDOT (2-way, vectors, FP16 to FP32)

Half-precision dot product to single-precision This instruction calculates the fused sum-of-products of a pair of half-precision values held in each 32-bit element of the first source and second source vectors, without intermediate rounding, and then destructively adds the single-precision sum-of-products to the corresponding single-precision element of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let elt1_a : bits(16) = operand1[(2 \* e + 0)\*:16]; let elt1_b : bits(16) = operand1[(2 \* e + 1)\*:16]; let elt2_a : bits(16) = operand2[(2 \* e + 0)\*:16]; let elt2_b : bits(16) = operand2[(2 \* e + 1)\*:16]; var sum : bits(32) = operand3[e\*:32]; sum = [FPDotAdd](shared_pseudocode.html#func_FPDotAdd_6)(sum, elt1_a, elt1_b, elt2_a, elt2_b, FPCR()); result[e\*:32] = sum; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
