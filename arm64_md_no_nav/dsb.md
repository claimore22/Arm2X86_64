## DSB

Data synchronization barrier

This instruction is a memory barrier that ensures the completion of memory accesses, see <span class="armarm-xref" data-linkend="ARMARM_BEICEFJH">Data Synchronization Barrier</span>.

This instruction is used by the aliases [PSSBB](pssbb_dsb.html "Physical speculative store bypass barrier"), and [SSBB](ssbb_dsb.html "Speculative store bypass barrier").

It has encodings from 2 classes: [Memory barrier](#iclass_memory_barrier) and [Memory nXS barrier](#iclass_memory_nxs_barrier)

### <span id="iclass_memory_barrier"></span>Memory barrier

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="4" class="lr">CRm</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="14"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="DSB_BO_barriers"></span>

DSB ([\<option\>](#CRm_option__2 "For the "Memory barrier" variant: specifies the limitation on the Memory Effects and maintenance operations ordered by")\|#[\<imm\>](#option__2 "Is a 4-bit unsigned immediate, in the range 0 to 15, encoded in the "CRm" field."))

</div>

#### Decode for this encoding

var nXS : boolean = FALSE; var alias : [DSBAlias](shared_pseudocode.html#type_DSBAlias); case CRm of when '0000' =\> alias = [DSBAlias_SSBB](shared_pseudocode.html#enum_DSBAlias_SSBB); when '0100' =\> alias = [DSBAlias_PSSBB](shared_pseudocode.html#enum_DSBAlias_PSSBB); otherwise =\> alias = [DSBAlias_DSB](shared_pseudocode.html#enum_DSBAlias_DSB); end; var scope : [MBMaintenanceScope](shared_pseudocode.html#type_MBMaintenanceScope); var types : [MBReqTypes](shared_pseudocode.html#type_MBReqTypes); case CRm\[1:0\] of when '00' =\> types = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); when '01' =\> types = [MBReqTypes_Reads](shared_pseudocode.html#enum_MBReqTypes_Reads); scope = [MBMaintenanceScope_None](shared_pseudocode.html#enum_MBMaintenanceScope_None); when '10' =\> types = [MBReqTypes_Writes](shared_pseudocode.html#enum_MBReqTypes_Writes); scope = [MBMaintenanceScope_None](shared_pseudocode.html#enum_MBMaintenanceScope_None); when '11' =\> types = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); case CRm\[3:2\] of when '00' =\> scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); when '01' =\> scope = [MBMaintenanceScope_Nonshareable](shared_pseudocode.html#enum_MBMaintenanceScope_Nonshareable); when '10' =\> scope = [MBMaintenanceScope_InnerShareable](shared_pseudocode.html#enum_MBMaintenanceScope_InnerShareable); when '11' =\> scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); end; end;

### <span id="iclass_memory_nxs_barrier"></span>Memory nXS barrier<span style="font-size:smaller;">\
(FEAT_XS) </span>

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="2" class="lr">imm2</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="14"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5" class="droppedname">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="DSB_BOn_barriers"></span>

DSB [\<option\>](#imm2_option "For the "Memory nXS barrier" variant: specifies the limitation on the barrier operation, and is ")nXS

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_XS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let types : [MBReqTypes](shared_pseudocode.html#type_MBReqTypes) = [MBReqTypes_All](shared_pseudocode.html#enum_MBReqTypes_All); var nXS : boolean = TRUE; let alias : [DSBAlias](shared_pseudocode.html#type_DSBAlias) = [DSBAlias_DSB](shared_pseudocode.html#enum_DSBAlias_DSB); var scope : [MBMaintenanceScope](shared_pseudocode.html#type_MBMaintenanceScope); case imm2 of when '00' =\> scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); when '01' =\> scope = [MBMaintenanceScope_Nonshareable](shared_pseudocode.html#enum_MBMaintenanceScope_Nonshareable); when '10' =\> scope = [MBMaintenanceScope_InnerShareable](shared_pseudocode.html#enum_MBMaintenanceScope_InnerShareable); when '11' =\> scope = [MBMaintenanceScope_OuterShareable](shared_pseudocode.html#enum_MBMaintenanceScope_OuterShareable); end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;option&gt;</td>
<td><span id="CRm_option__2"></span>
<p>For the "Memory barrier" variant: specifies the limitation on the Memory Effects and maintenance operations ordered by the barrier operation, and is encoded in "CRm".</p>
<div class="note">
<hr />
<h4 id="note">Note</h4>
<p>The value 0b0000 is used to encode <span class="asm-code">SSBB</span> and the value 0b0100 is used to encode <span class="asm-code">PSSBB</span>.</p>
<hr />
</div>
<p>For more information on whether an access is before or after a barrier instruction, see <span class="armarm-xref" data-linkend="BEICEFJH">Data Synchronization Barrier (DSB)</span>.</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">CRm</th>
<th class="symbol">&lt;option&gt;</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">OSHLD</td>
<td class="description"><p>This option has the same behavior as LD and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0010</td>
<td class="symbol">OSHST</td>
<td class="description"><p>This option has the same behavior as ST and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0011</td>
<td class="symbol">OSH</td>
<td class="description"><p>This option has the same behavior as SY.</p></td>
</tr>
<tr>
<td class="bitfield">0101</td>
<td class="symbol">NSHLD</td>
<td class="description"><p>This option has the same behavior as LD and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0110</td>
<td class="symbol">NSHST</td>
<td class="description"><p>This option has the same behavior as ST and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">0111</td>
<td class="symbol">NSH</td>
<td class="description"><p>Non-shareable is the required maintenance scope. Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">1x00</td>
<td class="symbol">RESERVED</td>
<td class="description"></td>
</tr>
<tr>
<td class="bitfield">1001</td>
<td class="symbol">ISHLD</td>
<td class="description"><p>This option has the same behavior as LD and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">1010</td>
<td class="symbol">ISHST</td>
<td class="description"><p>This option has the same behavior as ST and is deprecated.</p></td>
</tr>
<tr>
<td class="bitfield">1011</td>
<td class="symbol">ISH</td>
<td class="description"><p>Inner Shareable is the required maintenance scope. Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">1101</td>
<td class="symbol">LD</td>
<td class="description"><p>Reads are the required access type before the barrier instruction. Reads and writes are the required access types after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">1110</td>
<td class="symbol">ST</td>
<td class="description"><p>Writes are the required access type, both before and after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">1111</td>
<td class="symbol">SY</td>
<td class="description"><p>Outer Shareable is the required maintenance scope. Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
</tbody>
</table>
All encodings of "CRm" that are listed as reserved can be encoded using the #&lt;imm&gt; syntax. All reserved options have the same behavior as SY, but software must not rely on this behavior.</td>
</tr>
<tr>
<td></td>
<td><span id="imm2_option"></span>
<p>For the "Memory nXS barrier" variant: specifies the limitation on the barrier operation, and is encoded in “imm2”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm2</th>
<th class="symbol">&lt;option&gt;</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">OSH</td>
<td class="description"><p>This option has the same behavior as SY.</p></td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">NSH</td>
<td class="description"><p>Non-shareable is the required maintenance scope. Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">ISH</td>
<td class="description"><p>Inner Shareable is the required maintenance scope. Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">SY</td>
<td class="description"><p>Outer Shareable is the required maintenance scope. Reads and writes are the required access types, both before and after the barrier instruction.</p></td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="option__2"></span>
<p>Is a 4-bit unsigned immediate, in the range 0 to 15, encoded in the "CRm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [PSSBB](pssbb_dsb.html "Physical speculative store bypass barrier") | <span class="pseudocode">CRm == '0100'</span> |
| [SSBB](ssbb_dsb.html "Speculative store bypass barrier") | <span class="pseudocode">CRm == '0000'</span> |

<div class="ps">

### Operation

case alias of when [DSBAlias_SSBB](shared_pseudocode.html#enum_DSBAlias_SSBB) =\> SpeculativeStoreBypassBarrierToVA(); when [DSBAlias_PSSBB](shared_pseudocode.html#enum_DSBAlias_PSSBB) =\> SpeculativeStoreBypassBarrierToPA(); when [DSBAlias_DSB](shared_pseudocode.html#enum_DSBAlias_DSB) =\> if !nXS && IsFeatureImplemented(FEAT_XS) then nXS = [PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [IsHCRXEL2Enabled](shared_pseudocode.html#func_IsHCRXEL2Enabled_0)() && HCRX_EL2().FnXS == '1'; end; DataSynchronizationBarrier(scope, types, nXS); otherwise =\> unreachable; end;

</div>
