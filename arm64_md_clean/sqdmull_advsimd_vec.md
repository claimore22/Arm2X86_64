## SQDMULL, SQDMULL2 (vector)

Signed saturating doubling multiply long This instruction multiplies corresponding vector elements in the lower or upper half of the two source SIMD&FP registers, doubles the result, places the final results in a vector, and writes the vector to the destination SIMD&FP register. If overflow occurs with any of the results, those results are saturated. If saturation occurs, the cumulative saturation bit FPSR.QC is set. The SQDMULL instruction extracts each source vector from the lower half of each source register. The SQDMULL2 instruction extracts each source vector from the upper half of each source register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' \|\| size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = Vpart{}(n, part); let operand2 : bits(datasize) = Vpart{}(m, part); var result : bits(2\*datasize); var element1 : integer; var element2 : integer; var product : bits(2\*esize); var sat : boolean; for e = 0 to elements-1 do element1 = SInt(operand1[e\*:esize]); element2 = SInt(operand2[e\*:esize]); (product, sat) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){2\*esize}(2 \* element1 \* element2); result[e\*:(2\*esize)] = product; if sat then FPSR().QC = '1'; end; end; [V](shared_pseudocode.html#accessor_V_2){2\*datasize}(d) = result;
