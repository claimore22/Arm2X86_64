## TBX

Table vector lookup extension This instruction reads each value from the vector elements in the index source SIMD&FP register, uses each result as an index to perform a lookup in a table of bytes that is described by one to four source table SIMD&FP registers, places the lookup result in a vector, and writes the vector to the destination SIMD&FP register. If an index is out of range for the table, the existing value in the vector element of the destination register is left unchanged. If more than one source register is used to describe the table, the first source register describes the lowest bytes of the table. Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

#### Decode

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt(Rm); let datasize : integer{} = 64 << UInt(Q); let elements : integer = datasize DIV 8; let regs : integer{} = UInt(len) + 1; let is_tbl : boolean = (op == '0');

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let indices : bits(datasize) = V{}(m); var table : bits(128\*regs) = Zeros{}; var result : bits(datasize); var index : integer; // Create table from registers for i = 0 to regs - 1 do table[i\*:128] = [V](shared_pseudocode.html#accessor_V_2){128}((n+i) MOD 32); end; result = if is_tbl then Zeros{datasize} else [V](shared_pseudocode.html#accessor_V_2){datasize}(d); for i = 0 to elements - 1 do index = UInt(indices[i\*:8]); if index < 16 \* regs then result[i\*:8] = table[index\*:8]; end; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;
