## LUTI4 (four registers, 8-bit)

Lookup table read with 4-bit indexes and 8-bit elements (four registers) This instruction copies 8-bit elements from ZT0 to four destination vectors using packed 4-bit indices in the two source vectors. This instruction is unpredicated. It has encodings from 2 classes: [Consecutive](#iclass_consecutive) and [Strided](#iclass_strided)

#### Decode

if !IsFeatureImplemented(FEAT_SME2p1) \|\| !IsFeatureImplemented(FEAT_SME_LUTv2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let isize : integer{} = 4; let n : integer = UInt(Zn::'0'); let dstride : integer = 4; let d : integer = UInt(D::'00'::Zd); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let indexes : bits(2\*VL) = [Z](shared_pseudocode.html#accessor_Z_2){VL}(n+1) :: [Z](shared_pseudocode.html#accessor_Z_2){VL}(n+0); var dst : integer = d; let table : bits(512) = ZT0{}(); for r = 0 to nreg-1 do let base : integer = r \* elements; var result : bits(VL); for e = 0 to elements-1 do let index : integer = UInt(indexes[(base+e)\*:isize]); result[e\*:esize] = table[index\*:32][esize-1:0]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst) = result; dst = dst + dstride; end;
