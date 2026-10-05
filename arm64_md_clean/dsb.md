## DSB

Data synchronization barrier This instruction is a memory barrier that ensures the completion of memory accesses, see Data Synchronization Barrier. This instruction is used by the aliases [PSSBB](pssbb_dsb.html "Physical speculative store bypass barrier"), and [SSBB](ssbb_dsb.html "Speculative store bypass barrier"). It has encodings from 2 classes: [Memory barrier](#iclass_memory_barrier) and [Memory nXS barrier](#iclass_memory_nxs_barrier)

#### Decode

if !IsFeatureImplemented(FEAT_XS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let types : [MBReqTypes](shared_pseudocode.html#type_MBReqTypes) = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); var nXS : boolean = TRUE; let alias : [DSBAlias](shared_pseudocode.html#type_DSBAlias) = [DSBAlias_DSB](shared_pseudocode.html#enum_DSBAlias_DSB); var scope : [MBMaintenanceScope](shared_pseudocode.html#type_MBMaintenanceScope); case imm2 of when '00' => scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); when '01' => scope = [MBMaintenanceScope_Nonshareable](shared_pseudocode.html#enum_MBMaintenanceScope_Nonshareable); when '10' => scope = [MBMaintenanceScope_InnerShareable](shared_pseudocode.html#enum_MBMaintenanceScope_InnerShareable); when '11' => scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); end;

### Operation

case alias of when [DSBAlias_SSBB](shared_pseudocode.html#enum_DSBAlias_SSBB) => SpeculativeStoreBypassBarrierToVA(); when [DSBAlias_PSSBB](shared_pseudocode.html#enum_DSBAlias_PSSBB) => SpeculativeStoreBypassBarrierToPA(); when [DSBAlias_DSB](shared_pseudocode.html#enum_DSBAlias_DSB) => if !nXS && IsFeatureImplemented(FEAT_XS) then nXS = [PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [IsHCRXEL2Enabled](shared_pseudocode.html#func_IsHCRXEL2Enabled_0)() && HCRX_EL2().FnXS == '1'; end; DataSynchronizationBarrier(scope, types, nXS); otherwise => unreachable; end;
