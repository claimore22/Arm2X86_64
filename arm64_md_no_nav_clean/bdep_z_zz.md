## BDEP

Scatter lower bits into positions selected by bitmask This instruction scatters the lowest-numbered contiguous bits within each element of the first source vector to the bit positions indicated by non-zero bits in the corresponding mask element of the second source vector, preserving their order, and sets the bits corresponding to a zero mask bit to zero. This instruction is unpredicated. ID_AA64ZFR0_EL1.BitPerm indicates whether this instruction is implemented. This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled, or FEAT_SSVE_BitPerm is implemented.

#### Decode

if !IsFeatureImplemented(FEAT_SVE_BitPerm) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd);

### Operation

if IsFeatureImplemented(FEAT_SSVE_BitPerm) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let data : bits(VL) = Z{}(n); let mask : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements - 1 do result[e\*:esize] = [BitDeposit](shared_pseudocode.html#func_BitDeposit_3){esize}(data[e\*:esize], mask[e\*:esize]); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
