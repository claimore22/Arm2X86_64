## MOVA (vector to array, four registers)

Move Z four-vector operand to four ZA single-vector groups This instruction operates on four ZA single-vector groups. The single-vector group within each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo quarter the number of ZA array vectors. The vector group symbol VGx4 indicates that the instruction operates on four ZA single-vector groups. The preferred disassembly syntax uses a 64-bit element size, but an assembler should accept any element size if it is used consistently for all operands. The vector group symbol is preferred for disassembly, but optional in assembler source code. This instruction is unpredicated. This instruction is used by the alias [MOV (vector to array, four registers)](mov_mova_za_mz4.html "Move Z four-vector operand to four ZA single-vector groups").

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let offset : integer = UInt(off3); let n : integer = UInt(Zn::'00'); let nreg : integer{} = 4;

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; for r = 0 to nreg-1 do let result : bits(VL) = Z{}(n + r); [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = result; vec = vec + vstride; end;
