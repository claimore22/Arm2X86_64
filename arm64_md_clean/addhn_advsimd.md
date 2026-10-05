## ADDHN, ADDHN2

Add returning high narrow This instruction adds each vector element in the first source SIMD&FP register to the corresponding vector element in the second source SIMD&FP register, places the most significant half of the result into a vector, and writes the vector to the lower or upper half of the destination SIMD&FP register. The results are truncated. For rounded results, see RADDHN. The ADDHN instruction writes the vector to the lower half of the destination register and clears the upper half. The ADDHN2 instruction writes the vector to the upper half of the destination register without affecting the other bits of the register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize; let round : boolean = FALSE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(2\*datasize) = V{}(n); let operand2 : bits(2\*datasize) = V{}(m); var result : bits(datasize); var element1 : integer; var element2 : integer; var sum : integer; for e = 0 to elements-1 do element1 = UInt(operand1[e\*:(2\*esize)]); element2 = UInt(operand2[e\*:(2\*esize)]); sum = element1 + element2; sum = [RShr](shared_pseudocode.html#func_RShr_3)(sum, esize, round); result[e\*:esize] = sum[esize-1:0]; end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){datasize}(d, part) = result;
