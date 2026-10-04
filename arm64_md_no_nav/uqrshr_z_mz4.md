## UQRSHR (four registers)

Multi-vector unsigned saturating rounding shift right narrow by immediate

This instruction shifts right by an immediate value the unsigned integer value in each element of the four source vectors, and places the rounded results in the quarter-width destination elements. Each result element is saturated to the quarter-width N-bit element's unsigned integer range 0 to (2<sup>N</sup>)-1. The immediate shift amount is an unsigned value in the range 1 to number of bits per source element.

This instruction is unpredicated.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">tsize</td>
<td class="lr">1</td>
<td colspan="5" class="lr">imm5</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">N</td>
<td colspan="3"></td>
<td class="droppedname">op</td>
<td class="droppedname">U</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uqrshr_z_mz4_"></span>

UQRSHR [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__89 "Is the size specifier, "), { [\<Zn1\>](#Zn1__6 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").[\<Tb\>](#Tb__15 "Is the size specifier, ")-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").[\<Tb\>](#Tb__15 "Is the size specifier, ") }, \#[\<const\>](#const__11 "Is the immediate shift amount, in the range 1 to number of bits per source element, encoded in "tsize:imm5".")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if tsize == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< HighestSetBitNZ(tsize); let n : integer = UInt(Zn::'00'); let d : integer = UInt(Zd); let shift : integer = (8 \* esize) - UInt(tsize::imm5);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__89"></span>
<p>Is the size specifier, encoded in “tsize”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tsize</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">1x</td>
<td class="symbol">H</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__6"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Tb&gt;</td>
<td><span id="Tb__15"></span>
<p>Is the size specifier, encoded in “tsize”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tsize</th>
<th class="symbol">&lt;Tb&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">1x</td>
<td class="symbol">D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4__3"></span>
<p>Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;const&gt;</td>
<td><span id="const__11"></span>
<p>Is the immediate shift amount, in the range 1 to number of bits per source element, encoded in "tsize:imm5".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (4 \* esize); var result : bits(VL); for r = 0 to 3 do let operand : bits(VL) = Z{}(n+r); for e = 0 to elements-1 do let element : bits(4 \* esize) = operand\[e\*:(4 \* esize)\]; let res : integer = (UInt(element) + (1 \<\< (shift-1))) \>\> shift; result\[(r\*elements + e)\*:esize\] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(res); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
