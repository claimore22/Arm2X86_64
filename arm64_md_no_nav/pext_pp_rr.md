## PEXT (predicate pair)

Predicate extract pair from predicate-as-counter

This instruction converts the source predicate-as-counter into a four register wide predicate-as-mask, and copies the two portions of the mask value selected by the portion index to the two destination predicate registers. A portion corresponds to a one predicate register fraction of the wider predicate-as-mask value.

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
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">i1</td>
<td colspan="3" class="lr">PNn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pext_pp_rr_"></span>

PEXT { [\<Pd1\>](#Pd1 "Is the name of the first destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__67 "Is the size specifier, "), [\<Pd2\>](#Pd2 "Is the name of the second destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__67 "Is the size specifier, ") }, [\<PNn\>](#PNn__2 "Is the name of the first source scalable predicate register PN8-PN15, with predicate-as-counter encoding, encoded in the "PNn" field.")\[[\<imm\>](#imm__82 "Is the portion index, in the range 0 to 1, encoded in the "i1" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) && !IsFeatureImplemented(FEAT_SVE2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt('1'::PNn); let d0 : integer = UInt(Pd); let d1 : integer = (UInt(Pd) + 1) MOD 16; let part : integer = UInt(i1);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd1&gt;</td>
<td><span id="Pd1"></span>
<p>Is the name of the first destination scalable predicate register, encoded in the "Pd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__67"></span>
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
<td><span id="Pd2"></span>
<p>Is the name of the second destination scalable predicate register, encoded in the "Pd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;PNn&gt;</td>
<td><span id="PNn__2"></span>
<p>Is the name of the first source scalable predicate register PN8-PN15, with predicate-as-counter encoding, encoded in the "PNn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__82"></span>
<p>Is the portion index, in the range 0 to 1, encoded in the "i1" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_SVE2p1) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let pred : bits(PL) = P{}(n); let mask : bits(PL\*4) = CounterToPredicate{}(pred\[15:0\]); var result0 : bits(PL); var result1 : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL\*4}(mask, part \* 2 \* elements + e, esize); result0\[e\*:psize\] = ZeroExtend{psize}(pbit); end; for e = 0 to elements-1 do let pbit : bit = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL\*4}(mask, part \* 2 \* elements + elements + e, esize); result1\[e\*:psize\] = ZeroExtend{psize}(pbit); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d0) = result0; [P](shared_pseudocode.html#accessor_P_2){PL}(d1) = result1;

</div>
