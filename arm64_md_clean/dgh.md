## DGH

Data gathering hint This instruction is a hint instruction that indicates that it is not expected to be performance optimal to merge memory accesses with Normal Non-cacheable or Device-GRE attributes appearing in program order before the hint instruction with any memory accesses appearing after the hint instruction into a single memory transaction on an interconnect.

#### Decode

if !IsFeatureImplemented(FEAT_DGH) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

### Operation

Hint_DGH();
