## SETF8, SETF16

Evaluation of 8-bit or 16-bit flag values

This instruction sets the PSTATE.NZV flags based on the value in the specified general-purpose register. <span class="asm-code">SETF8</span> treats the value as an 8-bit value. <span class="asm-code">SETF16</span> treats the value as a 16-bit value.

The PSTATE.C flag is not affected by these instructions.

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">sz</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="6" class="droppedname">opcode2</td>
<td></td>
<td colspan="4"></td>
<td colspan="5"></td>
<td class="droppedname">o3</td>
<td colspan="4" class="droppedname">mask</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the SETF8 variant

<span id="SETF8_only_setf"></span> Applies when <span class="bitdiff"> (sz == 0)</span>

SETF8 [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the SETF16 variant

<span id="SETF16_only_setf"></span> Applies when <span class="bitdiff"> (sz == 1)</span>

SETF16 [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FlagM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = 8 \<\< UInt(sz); let n : integer = UInt(Rn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let reg : bits(32) = X{}(n); [PSTATE](shared_pseudocode.html#global_PSTATE).N = reg\[size-1\]; [PSTATE](shared_pseudocode.html#global_PSTATE).Z = if (reg\[size-1:0\] == Zeros{size}) then '1' else '0'; [PSTATE](shared_pseudocode.html#global_PSTATE).V = reg\[size\] XOR reg\[size-1\]; //PSTATE.C unchanged;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
