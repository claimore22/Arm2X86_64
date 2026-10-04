## UMULL, UMULL2 (by element)

Unsigned multiply long (vector, by element) This instruction multiplies each vector element in the lower or upper half of the first source SIMD&FP register by the specified vector element of the second source SIMD&FP register, places the results in a vector, and writes the vector to the destination SIMD&FP register. The destination vector elements are twice as long as the elements that are multiplied. The UMULL instruction extracts vector elements from the lower half of the first source register. The UMULL2 instruction extracts vector elements from the upper half of the first source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let idxdsize : integer{} = 64 << UInt(H); var index : integer; var Rmhi : bit; case size of when '01' => index = UInt(H::L::M); Rmhi = '0'; when '10' => index = UInt(H::L); Rmhi = M; otherwise => [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt(Rmhi::Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = Vpart{}(n, part); let operand2 : bits(idxdsize) = V{}(m); var result : bits(2\*datasize); var element1 : integer; let element2 : integer = UInt(operand2[index\*:esize]); var product : bits(2\*esize); for e = 0 to elements-1 do element1 = UInt(operand1[e\*:esize]); product = (element1 \* element2)[2\*esize-1:0]; result[e\*:(2\*esize)] = product; end; [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d) = result;
