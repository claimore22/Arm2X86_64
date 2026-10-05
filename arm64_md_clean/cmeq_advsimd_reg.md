## CMEQ (register)

Compare bitwise equal (vector) This instruction compares each vector element from the first source SIMD&FP register with the corresponding vector element from the second source SIMD&FP register, and if the comparison is equal sets every bit of the corresponding vector element in the destination SIMD&FP register to one, otherwise sets every bit of the corresponding vector element in the destination SIMD&FP register to zero. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size::Q == '110' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); var element1 : bits(esize); var element2 : bits(esize); for e = 0 to elements-1 do element1 = operand1[e\*:esize]; element2 = operand2[e\*:esize]; result[e\*:esize] = if element1 == element2 then Ones{esize} else Zeros{esize}; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
