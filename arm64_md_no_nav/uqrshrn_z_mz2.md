## UQRSHRN

Unsigned saturating rounding shift right narrow by immediate to interleaved integer

This instruction shifts right by an immediate value the unsigned integer value in each element of the two source vectors, and places the two-way interleaved rounded results in the half-width destination elements. Each result element is saturated to the half-width N-bit element's unsigned integer range 0 to (2<sup>N</sup>)-1. The immediate shift amount is an unsigned value in the range 1 to 16.

This instruction is unpredicated.

### <span id="iclass_16_bit"></span>16-bit<span style="font-size:smaller;">\
(FEAT_SME2 \|\| FEAT_SVE2p1) </span>

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
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="4" class="lr">imm4</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td class="droppedname">op0</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td class="droppedname">op1</td>
<td class="droppedname">U</td>
<td class="droppedname">R</td>
<td></td>
<td colspan="4"></td>
<td></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uqrshrn_z_mz2_"></span>

UQRSHRN [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").H, { [\<Zn1\>](#Zn1__4 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.").S-[\<Zn2\>](#Zn2__3 "Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.").S }, \#[\<const\>](#const__10 "Is the immediate shift amount, in the range 1 to 16, encoded in the "imm4" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 16; let n : integer = UInt(Zn::'0'); let d : integer = UInt(Zd); let shift : integer = esize - UInt(imm4);

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
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__4"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__3"></span>
<p>Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zn" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;const&gt;</td>
<td><span id="const__10"></span>
<p>Is the immediate shift amount, in the range 1 to 16, encoded in the "imm4" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV (2 \* esize); var result : bits(VL); for e = 0 to elements-1 do for i = 0 to 1 do let operand : bits(VL) = Z{}(n+i); let element : bits(2 \* esize) = operand\[e\*:(2 \* esize)\]; let res : integer = (UInt(element) + (1 \<\< (shift-1))) \>\> shift; result\[(2\*e + i)\*:esize\] = [UnsignedSat](shared_pseudocode.html#func_UnsignedSat_2){esize}(res); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>
