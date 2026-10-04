## BL

Branch with link

This instruction branches to a PC-relative offset, setting register X30 to PC+4. This instruction provides a hint that this is a subroutine call.

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
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="26" class="lr">imm26</td>
</tr>
<tr class="secondrow">
<td class="droppedname">op</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="26"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BL_only_branch_imm"></span>

BL [\<label\>](#imm26_offset "Is the program label to be unconditionally branched to. Its offset from the address of this instruction, in the range +/-128MB, is encoded as "imm26" times 4.")

</div>

#### Decode for this encoding

let offset : bits(64) = SignExtend{}(imm26::'00'); let d : integer = 30;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm26_offset"></span>
<p>Is the program label to be unconditionally branched to. Its offset from the address of this instruction, in the range +/-128MB, is encoded as "imm26" times 4.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then [AddGCSRecord](shared_pseudocode.html#func_AddGCSRecord_1)([PC64](shared_pseudocode.html#func_PC64_0)() + 4); end; [X](shared_pseudocode.html#accessor_X_2){64}(d) = [PC64](shared_pseudocode.html#func_PC64_0)() + 4; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIRCALL](shared_pseudocode.html#enum_BranchType_DIRCALL), branch_conditional);

</div>
