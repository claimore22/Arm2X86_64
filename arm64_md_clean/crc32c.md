## CRC32CB, CRC32CH, CRC32CW, CRC32CX

CRC32C checksum This instruction performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x1EDC6F41 is used for the CRC calculation. In an Armv8.0 implementation, this is an OPTIONAL instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.

#### Decode

if !IsFeatureImplemented(FEAT_CRC32) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); if sf == '1' && sz != '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && sz == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = 8 << UInt(sz);

### Operation

let acc : bits(32) = X{}(n); // accumulator let val : bits(size) = X{}(m); // input value let poly : bits(32) = 0x1EDC6F41[31:0]; let tempacc : bits(32+size) = [BitReverse](shared_pseudocode.html#func_BitReverse_2){32}(acc)::Zeros{size}; let tempval : bits(size+32) = [BitReverse](shared_pseudocode.html#func_BitReverse_2){size}(val)::Zeros{32}; // Poly32Mod2 on a bitstring does a polynomial Modulus over {0,1} operation [X](shared_pseudocode.html#accessor_X_2){32}(d) = [BitReverse](shared_pseudocode.html#func_BitReverse_2){32}([Poly32Mod2](shared_pseudocode.html#func_Poly32Mod2_3){32+size}(tempacc XOR tempval, poly));
