## ADDPT

Add checked pointer

This instruction adds a base address register value and an optionally-shifted register value, and writes the result to the destination register. The optionally-shifted register value is treated as the offset.

If the operation would have generated a result where the most significant 8 bits of the result register differ from the most significant 8 bits of the base register, then the result is modified such that it is likely to be non-canonical when used as an address.

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_CPA) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">imm3</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ADDPT_64_addsub_pt"></span>

ADDPT [\<Xd\|SP\>](#XdSP_option__3 "Is the 64-bit name of the general-purpose destination register or stack pointer, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__5 "Is the 64-bit name of the first general-purpose source register or stack pointer, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__4 "Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field."){, LSL \#[\<amount\>](#amount__3 "Is the left shift amount, in the range 0 to 7, defaulting to 0, encoded in the "imm3" field.")}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let shift : integer = UInt(imm3);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd|SP&gt;</td>
<td><span id="XdSP_option__3"></span>
<p>Is the 64-bit name of the general-purpose destination register or stack pointer, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__5"></span>
<p>Is the 64-bit name of the first general-purpose source register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__4"></span>
<p>Is the 64-bit name of the second general-purpose source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount__3"></span>
<p>Is the left shift amount, in the range 0 to 7, defaulting to 0, encoded in the "imm3" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var result : bits(64); let base : bits(64) = if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(n); let offset : bits(64) = LSL([X](shared_pseudocode.html#accessor_X_2){64}(m), shift); result = base + offset; result = [PointerAddCheck](shared_pseudocode.html#func_PointerAddCheck_2)(result, base); if d == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = result; else [X](shared_pseudocode.html#accessor_X_2){64}(d) = result; end;

</div>
