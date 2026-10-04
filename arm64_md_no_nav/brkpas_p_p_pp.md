## BRKPAS

Break after first true condition, propagating from previous partition and setting the condition flags

This instruction breaks after first true condition, propagating from previous partition, and sets the condition flags. If the Last active element of the first source predicate is false, then the destination predicate is set to all-false. Otherwise, the destination predicate elements up to and including the first active and true source element are set to true, then the subsequent elements are set to false. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

### <span id="iclass_setting_the_condition_flags"></span>Setting the condition flags<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Pm</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="4" class="lr">Pg</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">op</td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="4"></td>
<td class="droppedname">B</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="brkpas_p_p_pp_"></span>

BRKPAS [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").B, [\<Pg\>](#Pg__2 "Is the name of the governing scalable predicate register, encoded in the "Pg" field.")/Z, [\<Pn\>](#Pn__2 "Is the name of the first source scalable predicate register, encoded in the "Pn" field.").B, [\<Pm\>](#Pm__2 "Is the name of the second source scalable predicate register, encoded in the "Pm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let g : integer = UInt(Pg); let n : integer = UInt(Pn); let m : integer = UInt(Pm); let d : integer = UInt(Pd); let setflags : boolean = TRUE;

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
<td>&lt;Pg&gt;</td>
<td><span id="Pg__2"></span>
<p>Is the name of the governing scalable predicate register, encoded in the "Pg" field.</p></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(m); var result : bits(PL); var last : boolean = ([LastActive](shared_pseudocode.html#func_LastActive_4){PL}(mask, operand1, 8) == '1'); for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, 8) then let pbit : bit = if last then '1' else '0'; result\[e\*:1\] = ZeroExtend{1}(pbit); last = last && (\![ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, e, 8)); else result\[e\*:1\] = ZeroExtend{1}('0'); end; end; if setflags then PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the NZCV condition flags written by this instruction might be significantly delayed.
