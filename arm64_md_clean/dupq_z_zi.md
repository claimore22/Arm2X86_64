## DUPQ

Broadcast indexed element within each quadword vector segment (unpredicated) This instruction unconditionally broadcasts the indexed element within each 128-bit source vector segment to all elements of the corresponding destination vector segment. This instruction is unpredicated. The immediate element index is in the range of 0 to 15 (bytes), 7 (halfwords), 3 (words) or 1 (doublewords).

#### Decode

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if tsz == '0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let lsb : integer{} = LowestSetBit(tsz); let esize : integer{} = 8 << lsb; let imm : bits(5) = i1::tsz; let index : integer = UInt(imm[4:(lsb+1)]); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let segments : integer = VL DIV 128; let elements : integer = 128 DIV esize; let operand : bits(VL) = Z{}(n); var result : bits(VL); var element : bits(esize); for s = 0 to segments-1 do element = operand[(s \* elements + index)\*:esize]; result[s\*:128] = Replicate{128}(element); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
