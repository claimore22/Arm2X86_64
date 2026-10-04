## ESB

Error synchronization barrier

This instruction is an error synchronization event that might also update DISR_EL1 and VDISR_EL2.

This instruction can be used at all Exception levels and in Debug state.

In Debug state, this instruction behaves as if SError interrupts are masked at all Exception levels. For more information, see <span class="armarm-xref" data-linkend="ARMARM_RAS_pe_architecture">RAS PE architecture</span> and Arm<sup>®</sup> Reliability, Availability, and Serviceability (RAS) System Architecture, for A-profile architecture (ARM IHI 0100).

If <span class="armarm-xref" data-linkend="ARMARM_FEAT_RAS">FEAT_RAS</span> is not implemented, this instruction executes as a <span class="asm-code">NOP</span>.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_RAS) </span>

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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
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
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ESB_HI_hints"></span>

ESB

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_RAS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

SynchronizeErrors(); [AArch64_ESBOperation](shared_pseudocode.html#func_AArch64_ESBOperation_0)(); if [PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() then [AArch64_vESBOperation](shared_pseudocode.html#func_AArch64_vESBOperation_0)(); elsif IsFeatureImplemented(FEAT_E3DSE) && [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL3](shared_pseudocode.html#global_EL3) then [AArch64_dESBOperation](shared_pseudocode.html#func_AArch64_dESBOperation_0)(); end; TakeUnmaskedSErrorInterrupts();

</div>
