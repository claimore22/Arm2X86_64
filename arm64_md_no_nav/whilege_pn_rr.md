## WHILEGE (predicate as counter)

While decrementing signed scalar greater than or equal to scalar (predicate-as-counter)

This instruction generates a predicate for a group of two or four vectors that starting from the highest numbered element of the group is true while the decrementing value of the first, signed scalar operand is greater than or equal to the second scalar operand and false thereafter down to the lowest numbered element of the group.

If the second scalar operand is equal to the minimum signed integer value, then a condition that includes an equality test can never fail and the result will be an all-true predicate.

The full width of the scalar operands is significant for the purposes of comparison, and the full width first operand is decremented by one for each destination predicate element, irrespective of the predicate result element size.

The predicate result is placed in the predicate destination register using the predicate-as-counter encoding. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">vl</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="3" class="lr">PNd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td class="droppedname">U</td>
<td class="droppedname">lt</td>
<td colspan="5"></td>
<td></td>
<td class="droppedname">eq</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="whilege_pn_rr_"></span>

WHILEGE [\<PNd\>](#PNd "Is the name of the destination scalable predicate register PN8-PN15, with predicate-as-counter encoding, encoded in the "PNd" field.").[\<T\>](#T__72 "Is the size specifier, "), [\<Xn\>](#Xn__4 "Is the 64-bit name of the first source general-purpose register, encoded in the "Rn" field."), [\<Xm\>](#Xm__6 "Is the 64-bit name of the second source general-purpose register, encoded in the "Rm" field."), [\<vl\>](#vl "Is the vl specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let rsize : integer{} = 64; let n : integer = UInt(Rn); let m : integer = UInt(Rm); let d : integer = UInt('1'::PNd); let unsigned : boolean = FALSE; let invert : boolean = TRUE; let op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE); let width : integer = 2 \<\< UInt(vl);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;PNd&gt;</td>
<td><span id="PNd"></span>
<p>Is the name of the destination scalable predicate register PN8-PN15, with predicate-as-counter encoding, encoded in the "PNd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__72"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
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
<td>&lt;Xn&gt;</td>
<td><span id="Xn__4"></span>
<p>Is the 64-bit name of the first source general-purpose register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="Xm__6"></span>
<p>Is the 64-bit name of the second source general-purpose register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;vl&gt;</td>
<td><span id="vl"></span>
<p>Is the vl specifier, encoded in “vl”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">vl</th>
<th class="symbol">&lt;vl&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">VLx2</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">VLx4</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_SVE2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = width \* (VL DIV esize); var operand1 : bits(rsize) = X{}(n); let operand2 : bits(rsize) = X{}(m); var result : bits(PL); var last : boolean = TRUE; var count : integer = 0; let element2 : integer = if unsigned then UInt(operand2) else SInt(operand2); for e = elements-1 downto 0 do var cond : boolean; let element1 : integer = if unsigned then UInt(operand1) else SInt(operand1); case op of when [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT) =\> cond = (element1 \> element2); when [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE) =\> cond = (element1 \>= element2); end; last = last && cond; if last then count = count + 1; end; operand1 = operand1 - 1; end; result = [EncodePredCount](shared_pseudocode.html#func_EncodePredCount_5){PL}(esize, elements, count, invert); PSTATE.\[N,Z,C,V\] = [PredCountTest](shared_pseudocode.html#func_PredCountTest_3)(elements, count, invert); [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>
