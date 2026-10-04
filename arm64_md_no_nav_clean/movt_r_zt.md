## MOVT (table to scalar)

Move 8 bytes from ZT0 to general-purpose register This instruction moves 8 bytes to a general-purpose register from the ZT0 register at the byte offset specified by the immediate index. This instruction is UNDEFINED in Non-debug state.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) \|\| \![Halted](shared_pseudocode.html#func_Halted_0)() then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Rt); let offset : integer = UInt(off3);

### Operation

[CheckSMEEnabled](shared_pseudocode.html#func_CheckSMEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); let operand : bits(512) = ZT0{}(); [X](shared_pseudocode.html#accessor_X_2){64}(t) = operand[offset\*:64];
