## SQRDMULH (vector)

Signed saturating rounding doubling multiply returning high half This instruction multiplies the values of corresponding elements of the two source SIMD&FP registers, doubles the results, places the most significant half of the final results into a vector, and writes the vector to the destination SIMD&FP register. The results are rounded. For truncated results, see SQDMULH. If overflow occurs with any of the results, those results are saturated. If saturation occurs, the cumulative saturation bit FPSR.QC is set. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' \|\| size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize; let rounding : boolean = TRUE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); var element1 : integer; var element2 : integer; var product : integer; var sat : boolean; for e = 0 to elements-1 do element1 = SInt(operand1[e\*:esize]); element2 = SInt(operand2[e\*:esize]); product = 2 \* element1 \* element2; product = [RShr](shared_pseudocode.html#func_RShr_3)(product, esize, rounding); (result[e\*:esize], sat) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){esize}(product); if sat then FPSR().QC = '1'; end; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
