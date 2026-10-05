## FDOT (8-bit floating-point to half-precision, vector)

8-bit floating-point dot product to half-precision (vector) This instruction calculates the fused sum-of-products of a pair of 8-bit floating-point values held in each 16-bit element of the first and second source vectors. The half-precision sum-of-products are scaled by 2<sup>-UInt(FPMR.LSCALE[3:0])</sup>, before being destructively added without intermediate rounding to the corresponding half-precision elements of the destination vector. The 8-bit floating-point encoding format for the elements of the first source vector is selected by FPMR.F8S1. The 8-bit floating-point encoding format for the elements of the second source vector is selected by FPMR.F8S2.

#### Decode

if !IsFeatureImplemented(FEAT_FP8DOT2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = if Q == '1' then 128 else 64; let esize : integer{} = 16; let elements : integer = datasize DIV esize;

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); let operand3 : bits(datasize) = V{}(d); var result : bits(datasize); for e = 0 to elements-1 do let op1 : bits(esize) = operand1[e\*:esize]; let op2 : bits(esize) = operand2[e\*:esize]; var sum : bits(esize) = operand3[e\*:esize]; sum = [FP8DotAddFP](shared_pseudocode.html#func_FP8DotAddFP_7){esize, esize}(sum, op1, op2, FPCR(), FPMR()); result[e\*:esize] = sum; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
