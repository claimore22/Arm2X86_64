## MOVA (array to vector, two registers)

Move two ZA single-vector groups to Z two-vector operand This instruction operates on two ZA single-vector groups. The single-vector group within each half of the ZA array is selected by the sum of the vector select register and offset, modulo half the number of ZA array vectors. The vector group symbol VGx2 indicates that the instruction operates on two ZA single-vector groups. The preferred disassembly syntax uses a 64-bit element size, but an assembler should accept any element size if it is used consistently for all operands. The vector group symbol is preferred for disassembly, but optional in assembler source code. This instruction is unpredicated. This instruction is used by the alias [MOV (array to vector, two registers)](mov_mova_mz_za2.html "Move two ZA single-vector groups to Z two-vector operand").

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let offset : integer = UInt(off3); let d : integer = UInt(Zd::'0'); let nreg : integer{} = 2;

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; for r = 0 to nreg-1 do let result : bits(VL) = ZAvector{}(vec); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d + r) = result; vec = vec + vstride; end;
