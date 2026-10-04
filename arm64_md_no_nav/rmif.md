## RMIF

Rotate, mask insert flags

This instruction performs a rotation right of a value held in a general-purpose register by an immediate value, and then inserts a selection of the bottom four bits of the result of the rotation into the PSTATE flags, under the control of a second immediate mask.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_FlagM) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="6" class="lr">imm6</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">mask</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td class="droppedname">o2</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="RMIF_only_rmif"></span>

RMIF [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field."), \#[\<shift\>](#shift__9 "Is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field."), \#[\<mask\>](#mask "Is the flag bit mask, an immediate in the range 0 to 15, which selects the bits that are inserted into the NZCV condition flags, encoded in the "mask" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FlagM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let imm : integer{} = UInt(imm6); let flagmask : bits(4) = mask; let n : integer{} = UInt(Rn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="shift__9"></span>
<p>Is the shift amount, in the range 0 to 63, defaulting to 0 and encoded in the "imm6" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;mask&gt;</td>
<td><span id="mask"></span>
<p>Is the flag bit mask, an immediate in the range 0 to 15, which selects the bits that are inserted into the NZCV condition flags, encoded in the "mask" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let reg : bits(64) = X{}(n); let flags : bits(4) = (reg::reg)\[imm+3:imm\]; if flagmask\[3\] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).N = flags\[3\]; end; if flagmask\[2\] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).Z = flags\[2\]; end; if flagmask\[1\] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).C = flags\[1\]; end; if flagmask\[0\] == '1' then [PSTATE](shared_pseudocode.html#global_PSTATE).V = flags\[0\]; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
