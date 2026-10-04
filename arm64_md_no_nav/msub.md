## MSUB

Multiply-subtract

This instruction multiplies two register values, subtracts the product from a third register value, and writes the result to the destination register.

This instruction is used by the alias [MNEG](mneg_msub.html "Multiply-negate").

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
<td class="lr">sf</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Ra</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2" class="droppedname">op54</td>
<td colspan="5"></td>
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

#### Encoding for the 32-bit variant

<span id="MSUB_32A_dp_3src"></span> Applies when <span class="bitdiff"> (sf == 0)</span>

MSUB [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__5 "Is the 32-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__6 "Is the 32-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field."), [\<Wa\>](#WaOrWZR__2 "Is the 32-bit name of the third general-purpose source register holding the minuend, encoded in the "Ra" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="MSUB_64A_dp_3src"></span> Applies when <span class="bitdiff"> (sf == 1)</span>

MSUB [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__13 "Is the 64-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__9 "Is the 64-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field."), [\<Xa\>](#XaOrXZR__2 "Is the 64-bit name of the third general-purpose source register holding the minuend, encoded in the "Ra" field.")

</div>

#### Decode for all variants of this encoding

let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra); let datasize : integer{} = 32 \<\< UInt(sf);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR__5"></span>
<p>Is the 32-bit name of the first general-purpose source register holding the multiplicand, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__6"></span>
<p>Is the 32-bit name of the second general-purpose source register holding the multiplier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wa&gt;</td>
<td><span id="WaOrWZR__2"></span>
<p>Is the 32-bit name of the third general-purpose source register holding the minuend, encoded in the "Ra" field.</p></td>
</tr>
</tbody>
</table>

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
<td><span id="XaOrXZR__2"></span>
<p>Is the 64-bit name of the third general-purpose source register holding the minuend, encoded in the "Ra" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [MNEG](mneg_msub.html "Multiply-negate") | <span class="pseudocode">Ra == '11111'</span> |

<div class="ps">

### Operation

let operand1 : bits(datasize) = X{}(n); let operand2 : bits(datasize) = X{}(m); let operand3 : bits(datasize) = X{}(a); let result : integer = UInt(operand3) - (UInt(operand1) \* UInt(operand2)); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = result\[datasize-1:0\];

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
