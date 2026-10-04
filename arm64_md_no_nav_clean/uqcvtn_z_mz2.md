## UQCVTN

Unsigned 32-bit integer saturating extract narrow to interleaved 16-bit integer This instruction saturates the unsigned integer value in each element of the pair of source vectors to half the original source element width, and places the two-way interleaved results in the half-width destination elements. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); var result : bits(VL); for e = 0 to elements-1 do for i = 0 to 1 do let operand : bits(VL) = Z{}(n+i); let element : integer = UInt(operand[e\*:(2 \* esize)]); result[(2\*e + i)\*:esize] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(element); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
