## UQSHL (immediate)

Unsigned saturating shift left (immediate) This instruction takes each vector element in the source SIMD&FP register, shifts it by an immediate value, places the results in a vector, and writes the vector to the destination SIMD&FP register. The results are truncated. For rounded results, see UQRSHL. If overflow occurs with any of the results, those results are saturated. If saturation occurs, the cumulative saturation bit FPSR.QC is set. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh[3]::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << HighestSetBitNZ(immh); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = UInt(immh::immb) - esize; let src_unsigned : boolean = TRUE; let dst_unsigned : boolean = TRUE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var result : bits(datasize); var sat : boolean; for e = 0 to elements-1 do let opelt : bits(esize) = operand[e\*:esize]; let element : integer = if src_unsigned then UInt(opelt) else SInt(opelt); (result[e\*:esize], sat) = [SatQ](shared_pseudocode.html#func_SatQ_3){esize}(element << shift, dst_unsigned); if sat then FPSR().QC = '1'; end; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
