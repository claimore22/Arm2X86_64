## ADR

Calculate vector address This instruction optionally sign-extends or zero-extends the least significant 32 bits of each element from a vector of offsets or indices in the second source vector, and scales each index by 2, 4, or 8. The scaled indices are then added to a vector of base addresses from the first source vector, and the resulting addresses are placed in the destination vector. This instruction is unpredicated. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled. It has encodings from 3 classes: [Packed offsets](#iclass_packed_offsets) , [Unpacked 32-bit signed offsets](#iclass_unpacked_32_bit_signed_offsets) and [Unpacked 32-bit unsigned offsets](#iclass_unpacked_32_bit_unsigned_offsets)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd); let osize : integer{} = 32; let unsigned : boolean = TRUE; let mbytes : integer{} = 1 << UInt(msz);

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let base : bits(VL) = Z{}(n); let offs : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let addr : bits(esize) = base[e\*:esize]; let offselt : bits(osize) = offs[e\*:esize][osize-1:0]; let offset : integer = if unsigned then UInt(offselt) else SInt(offselt); result[e\*:esize] = addr + (offset \* mbytes); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
