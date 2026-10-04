## SETGP, SETGM, SETGE

Memory set with tag setting

These instructions set a requested number of bytes in memory to the value in the least significant byte of the source data register and store an Allocation Tag to memory for each Tag Granule written. The Allocation Tag is calculated from the Logical Address Tag in the register that holds the first address to be set. The prologue, main, and epilogue instructions are expected to be run in succession and to appear consecutively in memory: SETGP, then SETGM, and then SETGE.

SETGP performs some preconditioning of the arguments suitable for using the SETGM instruction, and sets an <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> portion of the requested number of bytes. SETGM sets a further <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> portion of the remaining bytes. SETGE sets any final remaining bytes.

<div class="note">

------------------------------------------------------------------------

#### Note

The ability to set an <span class="arm-defined-word">IMPLEMENTATION DEFINED</span> number of bytes allows an implementation to optimize how the bytes being set are divided between the different instructions.

------------------------------------------------------------------------

</div>

For more information on exceptions specific to memory set instructions, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.memcpy_and_memset_exceptions">Memory Copy and Memory Set exceptions</span>.

The architecture supports two algorithms for the memory set: option A and option B. Which algorithm is used is <span class="arm-defined-word">IMPLEMENTATION DEFINED</span>.

<div class="note">

------------------------------------------------------------------------

#### Note

Portable software should not assume that the choice of algorithm is constant.

------------------------------------------------------------------------

</div>

For SETGP:

- If Xn\[63\] == '1', the set size is saturated to 0x7FFFFFFFFFFFFFF0.

On completion of SETGP, option A:

- Xn holds -1 times the number of bytes in the saturated set size remaining to be set.
- Xd holds the original Xd + saturated set size.
- PSTATE.{N,Z,C,V} are set to {0,0,0,0}.

On completion of SETGP, option B:

- Xn holds the number of bytes in the saturated set size remaining to be set.
- Xd holds the lowest address that has not been set.
- PSTATE.{N,Z,C,V} are set to {0,0,1,0}.

For SETGM, option A, when PSTATE.C = '0':

- Xn holds a signed 64-bit integer.
- Xn holds -1 times the number of bytes remaining to be set.
- Xd holds the lowest address to be set - Xn.
- On completion of the instruction, Xn holds -1 times the number of bytes remaining to be set.

For SETGM, option B, when PSTATE.C = '1':

- Xn holds the number of bytes remaining to be set.
- Xd holds the lowest address to be set.
- On completion of the instruction:
  - Xn holds the number of bytes remaining to be set.
  - Xd holds the lowest address that has not been set.

For SETGE, option A, when PSTATE.C = '0':

- Xn holds a signed 64-bit integer.
- Xn holds -1 times the number of bytes remaining to be set.
- Xd holds the lowest address to be set - Xn.
- On completion of the instruction, Xn holds 0.

For SETGE, option B, when PSTATE.C = '1':

- Xn holds the number of bytes remaining to be set.
- Xd holds the lowest address to be set.
- On completion of the instruction:
  - Xn holds 0.
  - Xd holds the lowest address that has not been set.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_MOPS && FEAT_MTE) </span>

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td colspan="2" class="lr">sz</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rs</td>
<td class="l">x</td>
<td>x</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">o0</td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">op1</td>
<td></td>
<td colspan="5"></td>
<td colspan="4" class="droppedname">op2</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Prologue variant

<span id="SETGP_SET_memcms"></span> Applies when <span class="bitdiff"> (op2 == 0000)</span>

SETGP \[[\<Xd\>](#XdOrXZR__4 "For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds an encoding of the destination address (an integer multiple of 16) and is updated by the instruction, encoded in the "Rd" field.")\]!, [\<Xn\>](#XnOrXZR__8 "For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the number of bytes to be set (an integer multiple of 16) and is updated by the instruction, encoded in the "Rn" field.")!, [\<Xs\>](#XsOrXZR__8 "For the "Main" and "Prologue" variants: is the 64-bit name of the general-purpose register that holds the source data in bits<7:0>, encoded in the "Rs" field.")

</div>

<div class="encoding">

#### Encoding for the Main variant

<span id="SETGM_SET_memcms"></span> Applies when <span class="bitdiff"> (op2 == 0100)</span>

SETGM \[[\<Xd\>](#XdOrXZR__5 "For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the destination address (an integer multiple of 16) and for option B is updated by the instruction, encoded in the "Rd" field.")\]!, [\<Xn\>](#XnOrXZR__9 "For the "Main" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be set (an integer multiple of 16) and is updated by the instruction, encoded in the "Rn" field.")!, [\<Xs\>](#XsOrXZR__8 "For the "Main" and "Prologue" variants: is the 64-bit name of the general-purpose register that holds the source data in bits<7:0>, encoded in the "Rs" field.")

</div>

<div class="encoding">

#### Encoding for the Epilogue variant

<span id="SETGE_SET_memcms"></span> Applies when <span class="bitdiff"> (op2 == 1000)</span>

SETGE \[[\<Xd\>](#XdOrXZR__5 "For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the destination address (an integer multiple of 16) and for option B is updated by the instruction, encoded in the "Rd" field.")\]!, [\<Xn\>](#XnOrXZR__10 "For the "Epilogue" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be set (an integer multiple of 16) and is set to zero on completion of the instruction, encoded in the "Rn" field.")!, [\<Xs\>](#XsOrXZR__7 "For the "Epilogue" variant: is the 64-bit name of the general-purpose register that holds the source data, encoded in the "Rs" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_MOPS) \|\| !IsFeatureImplemented(FEAT_MTE) \|\| sz != '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var memset : [SETParams](shared_pseudocode.html#type_SETParams); memset.d = UInt(Rd); memset.s = UInt(Rs); memset.n = UInt(Rn); let options : bits(2) = op2\[1:0\]; let nontemporal : boolean = options\[1\] == '1'; case op2\[3:2\] of when '00' =\> memset.stage = [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue); when '01' =\> memset.stage = [MOPSStage_Main](shared_pseudocode.html#enum_MOPSStage_Main); when '10' =\> memset.stage = [MOPSStage_Epilogue](shared_pseudocode.html#enum_MOPSStage_Epilogue); otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

<div class="encoding-notes">

For information about the <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span> behavior of this instruction, see <span class="armarm-xref" data-linkend="CJAEGDJC">Architectural Constraints on UNPREDICTABLE behaviors</span>, and particularly <span class="armarm-xref" data-linkend="CEGDGDDAG2">Memory Copy and Memory Set SET\*</span> and <span class="armarm-xref" data-linkend="ARMARM_CEGCCGAE">Crossing a page boundary with different memory types or Shareability attributes</span>.

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__4"></span>
<p>For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds an encoding of the destination address (an integer multiple of 16) and is updated by the instruction, encoded in the "Rd" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XdOrXZR__5"></span>
<p>For the "Epilogue" and "Main" variants: is the 64-bit name of the general-purpose register that holds an encoding of the destination address (an integer multiple of 16) and for option B is updated by the instruction, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__8"></span>
<p>For the "Prologue" variant: is the 64-bit name of the general-purpose register that holds the number of bytes to be set (an integer multiple of 16) and is updated by the instruction, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XnOrXZR__9"></span>
<p>For the "Main" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be set (an integer multiple of 16) and is updated by the instruction, encoded in the "Rn" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XnOrXZR__10"></span>
<p>For the "Epilogue" variant: is the 64-bit name of the general-purpose register that holds an encoding of the number of bytes to be set (an integer multiple of 16) and is set to zero on completion of the instruction, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xs&gt;</td>
<td><span id="XsOrXZR__8"></span>
<p>For the "Main" and "Prologue" variants: is the 64-bit name of the general-purpose register that holds the source data in bits&lt;7:0&gt;, encoded in the "Rs" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="XsOrXZR__7"></span>
<p>For the "Epilogue" variant: is the 64-bit name of the general-purpose register that holds the source data, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckMOPSEnabled](shared_pseudocode.html#func_CheckMOPSEnabled_0)(); memset.is_setg = TRUE; [CheckSETConstrainedUnpredictable](shared_pseudocode.html#func_CheckSETConstrainedUnpredictable_1)(memset); let data : bits(8) = X{}(memset.s); var B : [MOPSBlockSize](shared_pseudocode.html#type_MOPSBlockSize) = 0; memset.nzcv = [PSTATE](shared_pseudocode.html#global_PSTATE).\[N,Z,C,V\]; memset.toaddress = [X](shared_pseudocode.html#accessor_X_2){64}(memset.d); if memset.stage == [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then memset.setsize = UInt([X](shared_pseudocode.html#accessor_X_2){64}(memset.n)); else memset.setsize = SInt([X](shared_pseudocode.html#accessor_X_2){64}(memset.n)); end; memset.implements_option_a = [SETGOptionA](shared_pseudocode.html#func_SETGOptionA_0)(); let privileged : boolean = (if options\[0\] == '1' then [AArch64_IsUnprivAccessPriv](shared_pseudocode.html#func_AArch64_IsUnprivAccessPriv_0)() else [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0)); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescSTGMOPS](shared_pseudocode.html#func_CreateAccDescSTGMOPS_2)(privileged, nontemporal); if memset.stage == [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then if memset.setsize \> [ArchMaxMOPSSETGSize](shared_pseudocode.html#global_ArchMaxMOPSSETGSize) then memset.setsize = [ArchMaxMOPSSETGSize](shared_pseudocode.html#global_ArchMaxMOPSSETGSize); end; if ((memset.setsize != 0 && !IsAlignedSize(memset.toaddress, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE))) \|\| !IsAlignedSize(memset.setsize\[63:0\], [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE))) then let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [AlignmentFault](shared_pseudocode.html#func_AlignmentFault_2)(accdesc, memset.toaddress); [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(fault); end; if memset.implements_option_a then memset.nzcv = '0000'; memset.toaddress = memset.toaddress + memset.setsize; memset.setsize = 0 - memset.setsize; else memset.nzcv = '0010'; end; end; memset.stagesetsize = [MemSetStageSize](shared_pseudocode.html#func_MemSetStageSize_1)(memset); if memset.stage != [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then [CheckMemSetParams](shared_pseudocode.html#func_CheckMemSetParams_2)(memset, options); var fault_address : bits(64); if memset.implements_option_a then fault_address = memset.toaddress + memset.setsize; else fault_address = memset.toaddress; end; if (memset.setsize != 0 && (memset.stagesetsize != 0 \|\| [MemStageSetZeroSizeCheck](shared_pseudocode.html#func_MemStageSetZeroSizeCheck_0)()) && !IsAlignedSize(memset.toaddress, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE))) then let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [AlignmentFault](shared_pseudocode.html#func_AlignmentFault_2)(accdesc, fault_address); [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(fault); end; if ((memset.stagesetsize != 0 \|\| [MemStageSetZeroSizeCheck](shared_pseudocode.html#func_MemStageSetZeroSizeCheck_0)()) && !IsAlignedSize(memset.setsize\[63:0\], [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE))) then let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [AlignmentFault](shared_pseudocode.html#func_AlignmentFault_2)(accdesc, fault_address); [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(fault); end; end; var tags_set : integer; var memaddrdesc : [AddressDescriptor](shared_pseudocode.html#type_AddressDescriptor); var memstatus : [PhysMemRetStatus](shared_pseudocode.html#type_PhysMemRetStatus); var tagmemaddrdesc : [AddressDescriptor](shared_pseudocode.html#type_AddressDescriptor); var tagmemstatus : [PhysMemRetStatus](shared_pseudocode.html#type_PhysMemRetStatus); var fault : boolean = FALSE; var tagfault : boolean = FALSE; if memset.implements_option_a then while memset.stagesetsize \< 0 && !fault && !tagfault looplimit [ArchMaxMOPSSETGSize](shared_pseudocode.html#global_ArchMaxMOPSSETGSize) do // IMP DEF selection of the block size that is worked on. While many // implementations might make this constant, that is not assumed. B = [SETSizeChoice](shared_pseudocode.html#func_SETSizeChoice_2)(memset, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)); assert B \<= -1 \* memset.stagesetsize && B\[3:0\] == '0000'; (-, memaddrdesc, memstatus) = [MemSetBytes](shared_pseudocode.html#func_MemSetBytes_4)(memset.toaddress + memset.setsize, data, B, accdesc); let tag : bits(4) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(memset.toaddress + memset.setsize); (tags_set, tagmemaddrdesc, tagmemstatus) = [MemSetTags](shared_pseudocode.html#func_MemSetTags_4)(memset.toaddress + memset.setsize, tag, B, accdesc); fault = [IsFault](shared_pseudocode.html#func_IsFault_1)(memaddrdesc) \|\| [IsFault](shared_pseudocode.html#func_IsFault_1)(memstatus); tagfault = [IsFault](shared_pseudocode.html#func_IsFault_1)(tagmemaddrdesc) \|\| [IsFault](shared_pseudocode.html#func_IsFault_1)(tagmemstatus); if !fault && !tagfault then memset.setsize = memset.setsize + B; memset.stagesetsize = memset.stagesetsize + B; end; end; else while memset.stagesetsize \> 0 && !fault && !tagfault looplimit [ArchMaxMOPSSETGSize](shared_pseudocode.html#global_ArchMaxMOPSSETGSize) do // IMP DEF selection of the block size that is worked on. While many // implementations might make this constant, that is not assumed. B = [SETSizeChoice](shared_pseudocode.html#func_SETSizeChoice_2)(memset, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)); assert B \<= memset.stagesetsize && B\[3:0\] == '0000'; (-, memaddrdesc, memstatus) = [MemSetBytes](shared_pseudocode.html#func_MemSetBytes_4)(memset.toaddress, data, B, accdesc); let tag : bits(4) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(memset.toaddress); (tags_set, tagmemaddrdesc, tagmemstatus) = [MemSetTags](shared_pseudocode.html#func_MemSetTags_4)(memset.toaddress, tag, B, accdesc); fault = [IsFault](shared_pseudocode.html#func_IsFault_1)(memaddrdesc) \|\| [IsFault](shared_pseudocode.html#func_IsFault_1)(memstatus); tagfault = [IsFault](shared_pseudocode.html#func_IsFault_1)(tagmemaddrdesc) \|\| [IsFault](shared_pseudocode.html#func_IsFault_1)(tagmemstatus); if !fault && !tagfault then memset.toaddress = memset.toaddress + B; memset.setsize = memset.setsize - B; memset.stagesetsize = memset.stagesetsize - B; end; end; end; [UpdateSetRegisters](shared_pseudocode.html#func_UpdateSetRegisters_3)(memset, fault \|\| tagfault, tags_set \* [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)); if fault then if [IsFault](shared_pseudocode.html#func_IsFault_1)(memaddrdesc) then [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(memaddrdesc.fault); else let iswrite : boolean = TRUE; [HandleExternalAbort](shared_pseudocode.html#func_HandleExternalAbort_5)(memstatus, iswrite, memaddrdesc, B, accdesc); end; elsif tagfault then if [IsFault](shared_pseudocode.html#func_IsFault_1)(tagmemaddrdesc) then [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(tagmemaddrdesc.fault); else let iswrite : boolean = TRUE; [HandleExternalAbort](shared_pseudocode.html#func_HandleExternalAbort_5)(tagmemstatus, iswrite, tagmemaddrdesc, 1, accdesc); end; end; if memset.stage == [MOPSStage_Prologue](shared_pseudocode.html#enum_MOPSStage_Prologue) then PSTATE.\[N,Z,C,V\] = memset.nzcv; end;

</div>
