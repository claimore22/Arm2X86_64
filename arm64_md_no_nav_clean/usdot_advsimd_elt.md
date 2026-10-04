## USDOT (by element)

Dot product with unsigned and signed integers (vector, by element) This instruction performs the dot product of the four unsigned 8-bit integer values in each 32-bit element of the first source register with the four signed 8-bit integer values in an indexed 32-bit element of the second source register, accumulating the result into the corresponding 32-bit element of the destination register. From Armv8.2 to Armv8.5, this is an OPTIONAL instruction. From Armv8.6 it is mandatory for implementations that include Advanced SIMD to support it. ID_AA64ISAR1_EL1.I8MM indicates whether this instruction is supported.

#### Decode

if !IsFeatureImplemented(FEAT_I8MM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt(M::Rm); let d : integer = UInt(Rd); let i : integer = UInt(H::L); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV 32;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(datasize) = V{}(d); var result : bits(datasize); for e = 0 to elements-1 do var res : bits(32) = operand3[e\*:32]; for b = 0 to 3 do let element1 : integer = UInt(operand1[(4 \* e + b)\*:8]); let element2 : integer = SInt(operand2[(4 \* i + b)\*:8]); res = res + element1 \* element2; end; result[e\*:32] = res; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
