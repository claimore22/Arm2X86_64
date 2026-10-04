## WHILEHS (predicate pair)

While decrementing unsigned scalar higher than or same as scalar (pair of predicates)

This instruction generates a pair of predicates that starting from the highest numbered element of the pair is true while the decrementing value of the first, unsigned scalar operand is higher than or the same as the second scalar operand and false thereafter down to the lowest numbered element of the pair.

If the second scalar operand is equal to the minimum unsigned integer value, then a condition that includes an equality test can never fail and the result will be an all-true predicate.

The full width of the scalar operands is significant for the purposes of comparison, and the full width first operand is decremented by one for each destination predicate element, irrespective of the predicate result element size. The first general-purpose source register is not itself updated.

The lower-numbered elements are placed in the first predicate destination register, and the higher-numbered elements in the second predicate destination register. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

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
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pd</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">U</td>
<td class="droppedname">lt</td>
<td colspan="5"></td>
<td></td>
<td colspan="3"></td>
<td class="droppedname">eq</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="whilehs_pp_rr_"></span>

WHILEHS { [\<Pd1\>](#Pd1__2 "Is the name of the first destination scalable predicate register, encoded as "Pd" times 2.").[\<T\>](#T__71 "Is the size specifier, "), [\<Pd2\>](#Pd2__2 "Is the name of the second destination scalable predicate register, encoded as "Pd" times 2 plus 1.").[\<T\>](#T__71 "Is the size specifier, ") }, [\<Xn\>](#Xn__4 "Is the 64-bit name of the first source general-purpose register, encoded in the "Rn" field."), [\<Xm\>](#Xm__6 "Is the 64-bit name of the second source general-purpose register, encoded in the "Rm" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let rsize : integer{} = 64; let n : integer = UInt(Rn); let m : integer = UInt(Rm); let d0 : integer = UInt(Pd::'0'); let d1 : integer = UInt(Pd::'1'); let unsigned : boolean = TRUE; let op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd1&gt;</td>
<td><span id="Pd1__2"></span>
<p>Is the name of the first destination scalable predicate register, encoded as "Pd" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__71"></span>
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
<td>&lt;Pd2&gt;</td>
<td><span id="Pd2__2"></span>
<p>Is the name of the second destination scalable predicate register, encoded as "Pd" times 2 plus 1.</p></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL\*2) = Ones{}; var operand1 : bits(rsize) = X{}(n); let operand2 : bits(rsize) = X{}(m); var result : bits(PL\*2); var last : boolean = TRUE; let psize : integer{} = esize DIV 8; let element2 : integer = if unsigned then UInt(operand2) else SInt(operand2); for e = (elements\*2)-1 downto 0 do let element1 : integer = if unsigned then UInt(operand1) else SInt(operand1); var cond : boolean; case op of when [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT) =\> cond = (element1 \> element2); when [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE) =\> cond = (element1 \>= element2); end; last = last && cond; let pbit : bit = if last then '1' else '0'; result\[e\*:psize\] = ZeroExtend{psize}(pbit); operand1 = operand1 - 1; end; PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL\*2}(mask, result, esize); [P](shared_pseudocode.html#accessor_P_2){PL}(d0) = result\[PL-1:0\]; [P](shared_pseudocode.html#accessor_P_2){PL}(d1) = result\[PL\*2-1:PL\];

</div>
