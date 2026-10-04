## PTRUES

Initialize predicate from named constraint and set the condition flags

This instruction sets elements of the destination predicate to true if the element number satisfies the named predicate constraint, or to false otherwise. If the constraint specifies more elements than are available at the current vector length, then all elements of the destination predicate are set to false.

The named predicate constraint limits the number of active elements in a single predicate to:

- A fixed number (VL1 to VL256)
- The largest power of two (POW2)
- The largest multiple of three or four (MUL3 or MUL4)
- All available, implicitly a multiple of two (ALL).

Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

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
<td colspan="2" class="lr">size</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">pattern</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td class="droppedname">S</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ptrues_p_s_"></span>

PTRUES [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__71 "Is the size specifier, "){, [\<pattern\>](#pattern "Is the optional pattern specifier, defaulting to ALL, ")}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let d : integer = UInt(Pd); let setflags : boolean = TRUE; let pat : bits(5) = pattern;

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
<td>&lt;pattern&gt;</td>
<td><span id="pattern"></span>
<p>Is the optional pattern specifier, defaulting to ALL, encoded in “pattern”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">pattern</th>
<th class="symbol">&lt;pattern&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00000</td>
<td class="symbol">POW2</td>
</tr>
<tr>
<td class="bitfield">00001</td>
<td class="symbol">VL1</td>
</tr>
<tr>
<td class="bitfield">00010</td>
<td class="symbol">VL2</td>
</tr>
<tr>
<td class="bitfield">00011</td>
<td class="symbol">VL3</td>
</tr>
<tr>
<td class="bitfield">00100</td>
<td class="symbol">VL4</td>
</tr>
<tr>
<td class="bitfield">00101</td>
<td class="symbol">VL5</td>
</tr>
<tr>
<td class="bitfield">00110</td>
<td class="symbol">VL6</td>
</tr>
<tr>
<td class="bitfield">00111</td>
<td class="symbol">VL7</td>
</tr>
<tr>
<td class="bitfield">01000</td>
<td class="symbol">VL8</td>
</tr>
<tr>
<td class="bitfield">01001</td>
<td class="symbol">VL16</td>
</tr>
<tr>
<td class="bitfield">01010</td>
<td class="symbol">VL32</td>
</tr>
<tr>
<td class="bitfield">01011</td>
<td class="symbol">VL64</td>
</tr>
<tr>
<td class="bitfield">01100</td>
<td class="symbol">VL128</td>
</tr>
<tr>
<td class="bitfield">01101</td>
<td class="symbol">VL256</td>
</tr>
<tr>
<td class="bitfield">0111x</td>
<td class="symbol">#uimm5</td>
</tr>
<tr>
<td class="bitfield">1xx00</td>
<td class="symbol">#uimm5</td>
</tr>
<tr>
<td class="bitfield">1x0x1</td>
<td class="symbol">#uimm5</td>
</tr>
<tr>
<td class="bitfield">1x010</td>
<td class="symbol">#uimm5</td>
</tr>
<tr>
<td class="bitfield">101x1</td>
<td class="symbol">#uimm5</td>
</tr>
<tr>
<td class="bitfield">10110</td>
<td class="symbol">#uimm5</td>
</tr>
<tr>
<td class="bitfield">11101</td>
<td class="symbol">MUL4</td>
</tr>
<tr>
<td class="bitfield">11110</td>
<td class="symbol">MUL3</td>
</tr>
<tr>
<td class="bitfield">11111</td>
<td class="symbol">ALL</td>
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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let pbit : bit = if e \< count then '1' else '0'; result\[e\*:psize\] = ZeroExtend{psize}(pbit); end; if setflags then PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(result, result, esize); end; [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the NZCV condition flags written by this instruction might be significantly delayed.
