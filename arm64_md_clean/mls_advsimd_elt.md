## MLS (by element)

Multiply-subtract from accumulator (vector, by element) This instruction multiplies the vector elements in the first source SIMD&FP register by the specified value in the second source SIMD&FP register, and subtracts the results from the vector elements of the destination SIMD&FP register. All the values in this instruction are unsigned integer values. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let idxdsize : integer{} = 64 << UInt(H); var index : integer; var Rmhi : bit; case size of when '01' => index = UInt(H::L::M); Rmhi = '0'; when '10' => index = UInt(H::L); Rmhi = M; otherwise => [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt(Rmhi::Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(idxdsize) = V{}(m); let operand3 : bits(datasize) = V{}(d); var result : bits(datasize); var element1 : integer; var element2 : integer; var product : bits(esize); element2 = UInt(operand2[index\*:esize]); for e = 0 to elements-1 do element1 = UInt(operand1[e\*:esize]); product = (element1 \* element2)[esize-1:0]; result[e\*:esize] = operand3[e\*:esize] - product; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
