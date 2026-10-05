## ZERO (table)

Zero ZT0 This instruction zeroes all bytes of the ZT0 register. This instruction does not require the PE to be in Streaming SVE mode, and it is expected that this instruction will not experience a significant slowdown due to contention with other PEs that are executing in Streaming SVE mode.

#### Decode

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

### Operation

[CheckSMEEnabled](shared_pseudocode.html#func_CheckSMEEnabled_0)(); [CheckSMEZT0Enabled](shared_pseudocode.html#func_CheckSMEZT0Enabled_0)(); [ZT0](shared_pseudocode.html#accessor_ZT0_1){512}() = Zeros{512};
