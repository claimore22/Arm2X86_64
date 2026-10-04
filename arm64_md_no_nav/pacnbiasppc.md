## PACNBIASPPC

Pointer Authentication Code for return address, using key A, not a branch target

This instruction calculates and inserts a Pointer Authentication Code for an instruction address, using two modifiers and key A.

The address is in X30.

The first modifier is in SP.

The second modifier is the value of PC.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_PAuth_LR) </span>

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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td></td>
<td class="droppedname">S</td>
<td></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="5" class="droppedname">opcode2</td>
<td colspan="6" class="droppedname">opcode</td>
<td colspan="5" class="droppedname">Rn</td>
<td colspan="5" class="droppedname">Rd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="PACNBIASPPC_64LR_dp_1src"></span>

PACNBIASPPC

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_PAuth_LR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = 30;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

[X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIA2](shared_pseudocode.html#func_AddPACIA2_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), [PC64](shared_pseudocode.html#func_PC64_0)());

</div>
