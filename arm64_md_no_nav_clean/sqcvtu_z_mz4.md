## SQCVTU (four registers)

Multi-vector signed saturating extract narrow to unsigned integer This instruction saturates the signed integer value in each element of the four source vectors to unsigned integer value that is quarter the original source element width, and places the results in the quarter-width destination elements. This instruction is unpredicated.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 << UInt(sz); let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd);

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (4 \* esize); var result : bits(VL); for r = 0 to 3 do let operand : bits(VL) = Z{}(n+r); for e = 0 to elements-1 do let element : integer = SInt(operand[e\*:(4 \* esize)]); result[(r\*elements + e)\*:esize] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(element); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;
