## UDOT (vector)

Dot product unsigned arithmetic (vector) This instruction performs the dot product of the four unsigned 8-bit elements in each 32-bit element of the first source register with the four unsigned 8-bit elements of the corresponding 32-bit element in the second source register, accumulating the result into the corresponding 32-bit element of the destination register. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped. In Armv8.2 and Armv8.3, this is an OPTIONAL instruction. From Armv8.4 it is mandatory for all implementations to support it.

#### Decode

if !IsFeatureImplemented(FEAT_DotProd) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size != '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let esize : integer{} = 8 << UInt(size); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV esize;

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize) = V{}(n); let operand2 : bits(datasize) = V{}(m); var result : bits(datasize); result = [V](shared_pseudocode.html#accessor_V_2){datasize}(d); for e = 0 to elements-1 do var res : integer = 0; var element1, element2 : integer; for i = 0 to 3 do element1 = UInt(operand1[(4 \* e + i)\*:(esize DIV 4)]); element2 = UInt(operand2[(4 \* e + i)\*:(esize DIV 4)]); res = res + element1 \* element2; end; result[e\*:esize] = result[e\*:esize] + res; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
