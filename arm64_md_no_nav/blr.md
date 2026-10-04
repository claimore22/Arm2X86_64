## BLR

Branch with link to register

This instruction calls a subroutine at an address in a register, setting register X30 to PC+4. This instruction provides a hint that this is a subroutine call.

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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">Z</td>
<td></td>
<td colspan="2" class="droppedname">op</td>
<td colspan="5" class="droppedname">op2</td>
<td colspan="4"></td>
<td class="droppedname">A</td>
<td class="droppedname">M</td>
<td colspan="5"></td>
<td colspan="5" class="droppedname">Rm</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BLR_64_branch_reg"></span>

BLR [\<Xn\>](#XnOrXZR "Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field.")

</div>

#### Decode for this encoding

let n : integer{} = UInt(Rn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR"></span>
<p>Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let target : bits(64) = X{}(n); if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then [AddGCSRecord](shared_pseudocode.html#func_AddGCSRecord_1)([PC64](shared_pseudocode.html#func_PC64_0)() + 4); end; // Value in BTypeNext will be used to set PSTATE.BTYPE [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '10'; [X](shared_pseudocode.html#accessor_X_2){64}(30) = [PC64](shared_pseudocode.html#func_PC64_0)() + 4; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_INDCALL](shared_pseudocode.html#enum_BranchType_INDCALL), branch_conditional);

</div>
