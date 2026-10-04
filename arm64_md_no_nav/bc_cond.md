## BC.cond

Branch consistent conditionally

This instruction branches conditionally to a label at a PC-relative offset, with a hint that this branch is very unlikely to change direction. This instruction provides a hint that this is not a subroutine call or return.

### <span id="iclass_19_bit_signed_pc_relative_branch_offset"></span>19-bit signed PC-relative branch offset<span style="font-size:smaller;">\
(FEAT_HBC) </span>

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
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="19" class="lr">imm19</td>
<td class="lr">1</td>
<td colspan="4" class="lr">cond</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="19"></td>
<td class="droppedname">o0</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BC_only_condbranch"></span>

BC.[\<cond\>](#cond_option "Is one of the standard conditions, encoded in the standard way, and ") [\<label\>](#imm19_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_HBC) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let offset : bits(64) = SignExtend{}(imm19::'00'); let condition : bits(4) = cond;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;cond&gt;</td>
<td><span id="cond_option"></span>
<p>Is one of the standard conditions, encoded in the standard way, and encoded in “cond”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cond</th>
<th class="symbol">&lt;cond&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0000</td>
<td class="symbol">EQ</td>
</tr>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">NE</td>
</tr>
<tr>
<td class="bitfield">0010</td>
<td class="symbol">CS</td>
</tr>
<tr>
<td class="bitfield">0011</td>
<td class="symbol">CC</td>
</tr>
<tr>
<td class="bitfield">0100</td>
<td class="symbol">MI</td>
</tr>
<tr>
<td class="bitfield">0101</td>
<td class="symbol">PL</td>
</tr>
<tr>
<td class="bitfield">0110</td>
<td class="symbol">VS</td>
</tr>
<tr>
<td class="bitfield">0111</td>
<td class="symbol">VC</td>
</tr>
<tr>
<td class="bitfield">1000</td>
<td class="symbol">HI</td>
</tr>
<tr>
<td class="bitfield">1001</td>
<td class="symbol">LS</td>
</tr>
<tr>
<td class="bitfield">1010</td>
<td class="symbol">GE</td>
</tr>
<tr>
<td class="bitfield">1011</td>
<td class="symbol">LT</td>
</tr>
<tr>
<td class="bitfield">1100</td>
<td class="symbol">GT</td>
</tr>
<tr>
<td class="bitfield">1101</td>
<td class="symbol">LE</td>
</tr>
<tr>
<td class="bitfield">1110</td>
<td class="symbol">AL</td>
</tr>
<tr>
<td class="bitfield">1111</td>
<td class="symbol">NV</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm19_offset"></span>
<p>Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range +/-1MB, is encoded as "imm19" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let branch_conditional : boolean = TRUE; if [ConditionHolds](shared_pseudocode.html#func_ConditionHolds_1)(condition) then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;

</div>
