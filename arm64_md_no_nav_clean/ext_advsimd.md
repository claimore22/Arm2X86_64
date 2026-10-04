## EXT

Extract vector from pair of vectors This instruction extracts the lowest vector elements from the second source SIMD&FP register and the highest vector elements from the first source SIMD&FP register, concatenates the results into a vector, and writes the vector to the destination SIMD&FP register vector. The index value specifies the lowest vector element to extract from the first source register, and consecutive elements are extracted from the first, then second, source registers until the destination vector is filled. The following figure shows an example of the operation of EXT doubleword operation for Q = 0 and imm4<2:0> = 3.\ ![EXT doubleword operation for Q = 0 and imm4<2:0> = 3](A64.ext_doubleword_operation_for_imm3.svg) Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Q == '0' && imm4[3] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let datasize : integer{} = 64 << UInt(Q); let position : integer{} = 8 \* UInt(imm4);

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let hi : bits(datasize) = V{}(m); let lo : bits(datasize) = V{}(n); let concat : bits(datasize\*2) = hi :: lo; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = concat[(position+datasize)-1:position];
