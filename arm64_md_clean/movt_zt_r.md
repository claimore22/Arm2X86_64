## MOVT (scalar to table)

Move 8 bytes from general-purpose register to ZT0 This instruction moves 8 bytes to the ZT0 register at the byte offset specified by the immediate index from a general-purpose register. This instruction is UNDEFINED in Non-debug state.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| \![Halted](shared_pseudocode.html#func_Halted_0)() then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Rt); let offset : integer = UInt(off3);

### Operation

[CheckSMEEnabled](shared_pseudocode.html#func_CheckSMEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); var result : bits(512) = ZT0{}(); result[offset\*:64] = [X](shared_pseudocode.html#accessor_X_2){64}(t); [ZT0](shared_pseudocode.html#accessor_ZT0_1){512}() = result;
