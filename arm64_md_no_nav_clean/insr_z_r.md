## INSR (scalar)

Insert general-purpose register in shifted vector This instruction shifts the destination vector left by one element, and then places a copy of the least-significant bits of the general-purpose register in element 0 of the destination vector. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let dn : integer = UInt(Zdn); let m : integer = UInt(Rm);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let dest : bits(VL) = Z{}(dn); let src : bits(esize) = X{}(m); [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = dest[(VL-esize)-1:0] :: src;
