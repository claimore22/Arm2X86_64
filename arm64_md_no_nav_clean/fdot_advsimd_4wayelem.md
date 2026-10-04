## FDOT (8-bit floating-point to single-precision, by element)

8-bit floating-point dot product to single-precision (vector, by element) This instruction calculates the fused sum-of-products of a group of four 8-bit floating-point values held in each 32-bit element of the first source vector and a group of four 8-bit floating-point values in an indexed 32-bit element of the second source vector. The single-precision sum-of-products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup>, before being destructively added without intermediate rounding to the corresponding single-precision elements of the destination vector. The 8-bit floating-point groups within the second source vector are specified using an immediate index. The 8-bit floating-point encoding format for the elements of the first source vector is selected by FPMR.F8S1. The 8-bit floating-point encoding format for the elements of the second source vector is selected by FPMR.F8S2.

#### Decode

if !IsFeatureImplemented(FEAT_FP8DOT4) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let d : integer = UInt(Rd); let m : integer = UInt(M::Rm); let i : integer = UInt(H::L); let datasize : integer{} = if Q == '1' then 128 else 64; let esize : integer{} = 32; let elements : integer = datasize DIV esize;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(datasize) = V{}(d); var result : bits(datasize); for e = 0 to elements-1 do let op1 : bits(esize) = operand1[e\*:esize]; let op2 : bits(esize) = operand2[i\*:esize]; let sum : bits(esize) = operand3[e\*:esize]; result[e\*:esize] = [FP8DotAddFP](shared_pseudocode.html#func_FP8DotAddFP_7){esize, esize}(sum, op1, op2, FPCR(), FPMR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
