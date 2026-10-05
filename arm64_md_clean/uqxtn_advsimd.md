## UQXTN, UQXTN2

Unsigned saturating extract narrow This instruction reads each vector element from the source SIMD&FP register, saturates each value to half the original width, places the result into a vector, and writes the vector to the destination SIMD&FP register. All the values in this instruction are unsigned integer values. If saturation occurs, the cumulative saturation bit FPSR.QC is set. The UQXTN instruction writes the vector to the lower half of the destination register and clears the upper half. The UQXTN2 instruction writes the vector to the upper half of the destination register without affecting the other bits of the register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize; let unsigned : boolean = TRUE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(2\*datasize) = V{}(n); var result : bits(datasize); var element : bits(2\*esize); var sat : boolean; for e = 0 to elements-1 do element = operand[e\*:(2\*esize)]; (result[e\*:esize], sat) = [SatQ](shared_pseudocode.html#func_SatQ_3){esize}(UInt(element), unsigned); if sat then FPSR().QC = '1'; end; end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){datasize}(d, part) = result;
