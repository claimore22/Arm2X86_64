## UQXTNB

Unsigned saturating extract narrow (bottom) This instruction saturates the unsigned integer value in each source element to half the original source element width, and places the results in the even-numbered half-width destination elements, while setting the odd-numbered elements to zero.

#### Decode

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let tsize : bits(3) = tszh::tszl; if !(tsize IN {'001', '010', '100'}) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16 << HighestSetBitNZ(tsize); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(n); var result : bits(VL); let halfesize : integer{} = esize DIV 2; for e = 0 to elements-1 do let element1 : integer = UInt(operand1[e\*:esize]); let res : bits(halfesize) = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){halfesize}(element1); result[(2\*e + 0)\*:halfesize] = res; result[(2\*e + 1)\*:halfesize] = Zeros{halfesize}; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
