## EXT

Extract vector from pair of vectors This instruction copies the indexed byte up to the last byte of the first source vector to the bottom of the result vector, then fills the remainder of the result starting from the first byte of the second source vector. The result is destructively placed in the destination and first source vector, or constructively placed in the destination vector. This instruction is unpredicated. An index that is greater than or equal to the vector length in bytes is treated as zero, resulting in the first source vector being copied to the result unchanged. It has encodings from 2 classes: [Constructive](#iclass_constructive) and [Destructive](#iclass_destructive)

#### Decode

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let dst : integer = UInt(Zdn); let s1 : integer = dst; let s2 : integer = UInt(Zm); let position : integer{} = UInt(imm8h::imm8l) \* 8;

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(s1); let operand2 : bits(VL) = Z{}(s2); var result : bits(VL); let concat : bits(VL\*2) = operand2 :: operand1; if position >= VL then result = concat[VL-1:0]; else result = concat[(position+VL)-1:position]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst) = result;
