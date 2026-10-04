## MADDPT

Multiply-add checked pointer

This instruction multiplies two register values, adds a third register value, and writes the result to the destination register. The intermediate product is treated as the offset.

If the operation would have generated a result where the most significant 8 bits of the result register differ from the most significant 8 bits of the base register, then the result is modified such that it is likely to be non-canonical when used as an address.

If the intermediate product cannot be correctly represented as a 64-bit two's complement value, then the result is modified such that it is likely to be non-canonical when used as an address.

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
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Ra</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td colspan="2" class="droppedname">op54</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="3" class="droppedname">op31</td>
<td colspan="5"></td>
<td class="droppedname">o0</td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="MADDPT_64A_dp_3src"></span>

MADDPT [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__13 "Is the 64-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__9 "Is the 64-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field."), [\<Xa\>](#XaOrXZR "Is the 64-bit name of the third general-purpose source register holding the addend, encoded in the "Ra" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__13"></span>
<p>Is the 64-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__9"></span>
<p>Is the 64-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xa&gt;</td>
<td><span id="XaOrXZR"></span>
<p>Is the 64-bit name of the third general-purpose source register holding the addend, encoded in the "Ra" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let operand1 : bits(64) = X{}(n); let operand2 : bits(64) = X{}(m); let operand3 : bits(64) = X{}(a); var result : bits(64); let product : integer = SInt(operand1) \* SInt(operand2); // Signed and unsigned twos complement arithmetic are equivalent if only a // fixed number of bits are considered. result = operand3 + product\[63:0\]; let overflow : boolean = (product != SInt(product\[63:0\])); result = [PointerMultiplyAddCheck](shared_pseudocode.html#func_PointerMultiplyAddCheck_3)(result, operand3, overflow); [X](shared_pseudocode.html#accessor_X_2){64}(d) = result;

</div>
