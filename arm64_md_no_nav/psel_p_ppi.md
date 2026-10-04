## PSEL

Predicate select between predicate register or all-false

This instruction places the contents of the first source predicate register into the destination predicate register if the indexed element of the second source predicate is true, otherwise it sets the destination predicate to all-false. The indexed element is determined by the sum of a general-purpose index register and an immediate, modulo the number of elements. This instruction does not set the condition flags. For programmer convenience, an assembler must also accept predicate-as-counter register names for the destination predicate register and the first source predicate register.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
(FEAT_SME \|\| FEAT_SVE2p1) </span>

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
<td class="lr">i1</td>
<td class="lr">tszh</td>
<td class="lr">1</td>
<td colspan="3" class="lr">tszl</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Pn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pm</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td class="droppedname">S</td>
<td colspan="4"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="psel_p_ppi_"></span>

PSEL [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field."), [\<Pn\>](#Pn__2 "Is the name of the first source scalable predicate register, encoded in the "Pn" field."), [\<Pm\>](#Pm__2 "Is the name of the second source scalable predicate register, encoded in the "Pm" field.").[\<T\>](#T__70 "Is the size specifier, ")\[[\<Wv\>](#Wv__2 "Is the 32-bit name of the vector select register W12-W15, encoded in the "Rv" field."), [\<imm\>](#imm__87 "Is the element index, in the range 0 to one less than the number of vector elements in a 128-bit vector register, encoded in "i1:tszh:tszl".")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let imm5 : bits(5) = i1::tszh::tszl; var esize : integer; var imm : integer; case tszh::tszl of when '0000' =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); when '1000' =\> esize = 64; imm = UInt(imm5\[4\]); when 'x100' =\> esize = 32; imm = UInt(imm5\[4:3\]); when 'xx10' =\> esize = 16; imm = UInt(imm5\[4:2\]); when 'xxx1' =\> esize = 8; imm = UInt(imm5\[4:1\]); end; let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let v : integer = UInt('011'::Rv);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd&gt;</td>
<td><span id="Pd"></span>
<p>Is the name of the destination scalable predicate register, encoded in the "Pd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pn&gt;</td>
<td><span id="Pn__2"></span>
<p>Is the name of the first source scalable predicate register, encoded in the "Pn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pm&gt;</td>
<td><span id="Pm__2"></span>
<p>Is the name of the second source scalable predicate register, encoded in the "Pm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__70"></span>
<p>Is the size specifier, encoded in “(tszh :: tszl)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tszh</th>
<th class="bitfield">tszl</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="bitfield">000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">x</td>
<td class="bitfield">xx1</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">x</td>
<td class="bitfield">x10</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">x</td>
<td class="bitfield">100</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="bitfield">000</td>
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
<td>&lt;Wv&gt;</td>
<td><span id="Wv__2"></span>
<p>Is the 32-bit name of the vector select register W12-W15, encoded in the "Rv" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__87"></span>
<p>Is the element index, in the range 0 to one less than the number of vector elements in a 128-bit vector register, encoded in "i1:tszh:tszl".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); let idx : bits(32) = X{}(v); let element : integer = (UInt(idx) + imm) MOD elements; var result : bits(PL); if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, element, esize) then result = operand1; else result = Zeros{PL}; end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
