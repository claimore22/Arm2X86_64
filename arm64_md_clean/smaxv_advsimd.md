## SMAXV

Signed maximum across vector This instruction compares all the vector elements in the source SIMD&FP register, and writes the largest of the values as a scalar to the destination SIMD&FP register. All the values in this instruction are signed integer values. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size::Q == '100' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize; let unsigned : boolean = FALSE;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var opelt : bits(esize) = operand[0+:esize]; var max : integer = if unsigned then UInt(opelt) else SInt(opelt); for e = 1 to elements-1 do opelt = operand[e\*:esize]; let element : integer = if unsigned then UInt(opelt) else SInt(opelt); max = Max(max, element); end; [V](shared_pseudocode.html#accessor_V_2){esize}(d) = max[esize-1:0];
