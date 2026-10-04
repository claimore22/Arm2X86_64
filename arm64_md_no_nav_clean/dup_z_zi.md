## DUP (indexed)

Broadcast indexed element to vector (unpredicated) This instruction unconditionally broadcasts the indexed source vector element into each element of the destination vector. This instruction is unpredicated. The immediate element index is in the range of 0 to 63 (bytes), 31 (halfwords), 15 (words), 7 (doublewords) or 3 (quadwords). Selecting an element beyond the accessible vector length causes the destination vector to be set to zero. This instruction is used by the alias [MOV (SIMD&FP scalar, unpredicated)](mov_dup_z_zi.html "Move indexed element or SIMD&FP scalar to vector (unpredicated)").

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if tsz == '00000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let lsb : integer{} = LowestSetBit(tsz); let esize : integer{} = 8 << lsb; let imm : bits(7) = imm2::tsz; let index : integer = UInt(imm[6:(lsb+1)]); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = if index < elements then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL); var element : bits(esize); if index >= elements then element = Zeros{esize}; else element = operand1[index\*:esize]; end; result = Replicate{VL}(element); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
