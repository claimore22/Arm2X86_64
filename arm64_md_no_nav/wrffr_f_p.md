## WRFFR

Write the first-fault register

This instruction reads the source predicate register, and writes the value in the register to the first-fault register (FFR). This instruction is intended to restore a saved FFR and is not recommended for general use by applications.

This instruction requires that the source predicate contains a monotonic predicate value, in which starting from bit 0 there are zero or more 1 bits, followed only by 0 bits in any remaining bit positions. If the source is not a monotonic predicate value, then the resulting value in the FFR will be <span class="arm-defined-word">UNPREDICTABLE</span>. It is not possible to generate a non-monotonic value in FFR when using <span class="asm-code">SETFFR</span> followed by first-fault or non-fault loads.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
(FEAT_SVE) </span>

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="4" class="lr">Pn</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td colspan="3"></td>
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="wrffr_f_p_"></span>

WRFFR [\<Pn\>](#Pn__3 "Is the name of the source scalable predicate register, encoded in the "Pn" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Pn);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pn&gt;</td>
<td><span id="Pn__3"></span>
<p>Is the name of the source scalable predicate register, encoded in the "Pn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let operand : bits(PL) = P{}(n); let hsb : integer{} = HighestSetBit(operand); if hsb \< 0 \|\| IsOnes(operand\[hsb:0\]) then [FFR](shared_pseudocode.html#accessor_FFR_1){PL}() = operand; else // not a monotonic predicate [FFR](shared_pseudocode.html#accessor_FFR_1){PL}() = ARBITRARY : bits(PL); end;

</div>
