## BTI

Branch target identification

This instruction is used to guard against the execution of instructions that are not the intended target of a branch.

Outside of a guarded memory region, a <span class="asm-code">BTI</span> instruction executes as a <span class="asm-code">NOP</span>. Within a guarded memory region, while <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.BTYPE != 0b00, a <span class="asm-code">BTI</span> instruction compatible with the current value of PSTATE.BTYPE will not generate a Branch Target Exception and will allow execution of subsequent instructions within the memory region. For more information, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.PSTATE_BTYPE">PSTATE.BTYPE</span>.

The operand \<targets\> passed to a <span class="asm-code">BTI</span> instruction determines the values of <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.BTYPE that the <span class="asm-code">BTI</span> instruction is compatible with.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_BTI) </span>

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
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">x</td>
<td>x</td>
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

<span id="BTI_HB_hints"></span>

BTI {[\<targets\>](#targets_option "Is the type of indirection, ")}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_BTI) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; // Check branch target compatibility between BTI instruction and PSTATE.BTYPE [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)([BTypeCompatible_BTI](shared_pseudocode.html#func_BTypeCompatible_BTI_1)(op2\[2:1\]));

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;targets&gt;</td>
<td><span id="targets_option"></span>
<p>Is the type of indirection, encoded in “op2[2:1]”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op2[2:1]</th>
<th class="symbol">&lt;targets&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">[absent]</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">c</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">j</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">jc</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[SetBTypeNext](shared_pseudocode.html#func_SetBTypeNext_1)('00');

</div>
