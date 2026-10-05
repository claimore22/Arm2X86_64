## UZP1

Unzip vectors (primary) This instruction reads corresponding even-numbered vector elements from the two source SIMD&FP registers, starting at zero, places the result from the first source register into consecutive elements in the lower half of a vector, and the result from the second source register into consecutive elements in the upper half of a vector, and writes the vector to the destination SIMD&FP register.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size::Q == '110' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize; let part : integer = UInt(op);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operandl : bits(datasize) = V{}(n); let operandh : bits(datasize) = V{}(m); var result : bits(datasize); let zipped : bits(datasize\*2) = operandh::operandl; for e = 0 to elements-1 do result[e\*:esize] = zipped[(2\*e+part)\*:esize]; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
