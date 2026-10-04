## TSB

Trace synchronization barrier

This instruction is a barrier that synchronizes the trace operations of instructions, see <span class="armarm-xref" data-linkend="ARMARM_BEIJJEGJ">Trace Synchronization Barrier (TSB)</span>.

If <span class="armarm-xref" data-linkend="ARMARM_FEAT_TRF">FEAT_TRF</span> is not implemented, this instruction executes as a <span class="asm-code">NOP</span>.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_TRF) </span>

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
<td>1</td>
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

<span id="TSB_HC_hints"></span>

TSB CSYNC

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_TRF) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_FGT2) && IsFeatureImplemented(FEAT_TRBEv1p1) then let trap_to_el2 : boolean = ([PSTATE](shared_pseudocode.html#global_PSTATE).EL IN {[EL0](shared_pseudocode.html#global_EL0), [EL1](shared_pseudocode.html#global_EL1)} && [EL2Enabled](shared_pseudocode.html#func_EL2Enabled_0)() && \![IsInHost](shared_pseudocode.html#func_IsInHost_0)() && (\![HaveEL](shared_pseudocode.html#func_HaveEL_1)([EL3](shared_pseudocode.html#global_EL3)) \|\| SCR_EL3().FGTEn2 == '1') && HFGITR2_EL2().TSBCSYNC == '1'); if trap_to_el2 then let target_el : bits(2) = [EL2](shared_pseudocode.html#global_EL2); let iss : bits(25) = 0x4\[24:0\]; [AArch64_OtherInstrTrap](shared_pseudocode.html#func_AArch64_OtherInstrTrap_2)(target_el, iss); end; end; [TraceSynchronizationBarrier](shared_pseudocode.html#func_TraceSynchronizationBarrier_0)();

</div>
