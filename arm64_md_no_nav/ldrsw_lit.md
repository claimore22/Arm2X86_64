## LDRSW (literal)

Load register signed word (literal)

This instruction calculates an address from the PC value and an immediate offset, loads a word from memory, and writes it to a register. For information about addressing modes, see <span class="armarm-xref" data-linkend="ARMARM_CHDIIIBB">Load/Store addressing modes</span>.

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
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="19" class="lr">imm19</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td></td>
<td colspan="19"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="LDRSW_64_loadlit"></span>

LDRSW [\<Xt\>](#XtOrXZR__8 "Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field."), [\<label\>](#imm19_offset__2 "Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.")

</div>

#### Decode for this encoding

let t : integer{} = UInt(Rt); let nontemporal : boolean = FALSE; let tagchecked : boolean = FALSE; let offset : bits(64) = SignExtend{}(imm19::'00');

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__8"></span>
<p>Is the 64-bit name of the general-purpose register to be loaded, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm19_offset__2"></span>
<p>Is the program label from which the data is to be loaded. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let address : bits(64) = [PC64](shared_pseudocode.html#func_PC64_0)() + offset; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescGPR](shared_pseudocode.html#func_CreateAccDescGPR_5)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, privileged, tagchecked, t); let data : bits(32) = [Mem](shared_pseudocode.html#accessor_Mem_3){32}(address, accdesc); [X](shared_pseudocode.html#accessor_X_2){64}(t) = SignExtend{64}(data);

</div>
