## BRKNS

Propagate break to next partition, setting the condition flags

This instruction propagates break to next partition and sets the condition flags. If the Last active element of the first source predicate is false, then the destination predicate is set to all-false. Otherwise, the destination and second source predicates are left unchanged. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

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
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Pg</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pdm</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="4"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="brkns_p_p_pp_"></span>

BRKNS [\<Pdm\>](#Pdm "Is the name of the second source and destination scalable predicate register, encoded in the "Pdm" field.").B, [\<Pg\>](#Pg__2 "Is the name of the governing scalable predicate register, encoded in the "Pg" field.")/Z, [\<Pn\>](#Pn__2 "Is the name of the first source scalable predicate register, encoded in the "Pn" field.").B, [\<Pdm\>](#Pdm "Is the name of the second source and destination scalable predicate register, encoded in the "Pdm" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let g : integer = UInt(Pg); let n : integer = UInt(Pn); let dm : integer = UInt(Pdm); let setflags : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pdm&gt;</td>
<td><span id="Pdm"></span>
<p>Is the name of the second source and destination scalable predicate register, encoded in the "Pdm" field.</p></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let mask : bits(PL) = P{}(g); let operand1 : bits(PL) = P{}(n); let operand2 : bits(PL) = P{}(dm); var result : bits(PL); if [LastActive](shared_pseudocode.html#func_LastActive_4){PL}(mask, operand1, 8) == '1' then result = operand2; else result = Zeros{PL}; end; if setflags then PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(Ones{PL}, result, 8); end; [P](shared_pseudocode.html#accessor_P_2){PL}(dm) = result;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the NZCV condition flags written by this instruction might be significantly delayed.
