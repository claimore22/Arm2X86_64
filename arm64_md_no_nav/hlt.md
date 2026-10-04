## HLT

Halt instruction

This instruction can generate a Halt Instruction debug event, which causes entry into Debug state.

Within a guarded memory region, while <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.BTYPE != 0b00, an <span class="asm-code">HLT</span> instruction that would cause entry into Debug state will not generate a Branch Target exception and will cause entry into Debug state as normal. For more information, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.PSTATE_BTYPE">PSTATE.BTYPE</span>.

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
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="16" class="lr">imm16</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">opc</td>
<td colspan="16"></td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="2" class="droppedname">LL</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="HLT_EX_exception"></span>

HLT \#[\<imm\>](#imm "Is a 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.")

</div>

#### Decode for this encoding

if EDSCR().HDE == '0' \|\| \![HaltingAllowed](shared_pseudocode.html#func_HaltingAllowed_0)() then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); elsif IsFeatureImplemented(FEAT_BTI) then [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)(TRUE); end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm"></span>
<p>Is a 16-bit unsigned immediate, in the range 0 to 65535, encoded in the "imm16" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let is_async : boolean = FALSE; let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [NoFault](shared_pseudocode.html#func_NoFault_0)(); [Halt](shared_pseudocode.html#func_Halt_3)([DebugHalt_HaltInstruction](shared_pseudocode.html#global_DebugHalt_HaltInstruction), is_async, fault);

</div>
