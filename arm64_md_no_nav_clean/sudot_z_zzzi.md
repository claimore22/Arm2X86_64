## SUDOT

Signed by unsigned 8-bit integer dot product by indexed element to 32-bit integer This instruction calculates the dot product of a group of four signed 8-bit integer values held in each 32-bit element of the first source vector multiplied by a group of four unsigned 8-bit integer values in an indexed 32-bit element of the second source vector, and then destructively adds the widened dot product to the corresponding 32-bit element of the destination vector. The groups within the second source vector are specified using an immediate index that selects the same group position within each 128-bit vector segment. The index range is from 0 to 3. This instruction is unpredicated. ID_AA64ZFR0_EL1.I8MM indicates whether this instruction is implemented.

#### Decode

if ((!IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME)) \|\| !IsFeatureImplemented(FEAT_I8MM)) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let index : integer = UInt(i2); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let da : integer = UInt(Zda);

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let eltspersegment : integer = 128 DIV esize; let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(da); var result : bits(VL); for e = 0 to elements-1 do let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = segmentbase + index; var res : bits(esize) = operand3[e\*:esize]; for i = 0 to 3 do let element1 : integer = SInt(operand1[(4 \* e + i)\*:(esize DIV 4)]); let element2 : integer = UInt(operand2[(4 \* s + i)\*:(esize DIV 4)]); res = res + element1 \* element2; end; result[e\*:esize] = res; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(da) = result;
