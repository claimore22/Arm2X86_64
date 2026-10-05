## SUBP

Subtract pointer This instruction subtracts the 56-bit address held in the second source register from the 56-bit address held in the first source register, sign-extends the result to 64 bits, and writes the result to the destination register.

#### Decode

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

### Operation

var operand1 : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); var operand2 : bits(64) = if m == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(m); operand1 = SignExtend{64}(operand1[55:0]); operand2 = NOT(SignExtend{64}(operand2[55:0])); var result : bits(64); (result, -) = [AddWithCarry](shared_pseudocode.html#func_AddWithCarry_4){64}(operand1, operand2, '1'); [X](shared_pseudocode.html#accessor_X_2){64}(d) = result;
