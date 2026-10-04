## MADPT

Multiply-add checked pointer vectors to multiplicand

This instruction multiplies, with overflow check, the elements of the first and second source vectors and adds the results of multiplication, with pointer check, to elements of the third (addend) vector. The final results are destructively placed in the destination and first source (multiplicand) vector.

This instruction is illegal when executed in Streaming SVE mode, unless FEAT_SME_FA64 is implemented and enabled.

### <span id="iclass_sve2"></span>SVE2<span style="font-size:smaller;">\
(FEAT_SVE && FEAT_CPA) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Za</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="madpt_z_zzz_"></span>

MADPT [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").D, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").D, [\<Za\>](#Za "Is the name of the third source scalable vector register, encoded in the "Za" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) \|\| !IsFeatureImplemented(FEAT_CPA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let dn : integer = UInt(Zdn); let m : integer = UInt(Zm); let a : integer = UInt(Za);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn&gt;</td>
<td><span id="Zdn"></span>
<p>Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Za&gt;</td>
<td><span id="Za"></span>
<p>Is the name of the third source scalable vector register, encoded in the "Za" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 64; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(a); var result : bits(VL); for e = 0 to elements-1 do let element1 : integer = SInt(operand1\[e\*:64\]); let element2 : integer = SInt(operand2\[e\*:64\]); let product : integer = element1 \* element2; let overflow : boolean = (product != SInt(product\[63:0\])); let addend : bits(64) = operand3\[e\*:64\]; result\[e\*:64\] = [PointerMultiplyAddCheck](shared_pseudocode.html#func_PointerMultiplyAddCheck_3)(addend + product, addend, overflow); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
