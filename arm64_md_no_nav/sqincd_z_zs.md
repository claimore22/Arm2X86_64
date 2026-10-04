## SQINCD (vector)

Signed saturating increment vector by multiple of 64-bit predicate constraint element count

This instruction determines the number of active 64-bit elements implied by the named predicate constraint, multiplies that by an immediate in the range 1 to 16 inclusive, and then uses the result to increment all destination vector elements. The results are saturated to the 64-bit signed integer range.

The named predicate constraint limits the number of active elements in a single predicate to:

- A fixed number (VL1 to VL256)
- The largest power of two (POW2)
- The largest multiple of three or four (MUL3 or MUL4)
- All available, implicitly a multiple of two (ALL).

Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception.

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
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
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="4" class="lr">imm4</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">pattern</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">D</td>
<td class="droppedname">U</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sqincd_z_zs_"></span>

SQINCD [\<Zdn\>](#Zdn__2 "Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.").D{, [\<pattern\>](#pattern "Is the optional pattern specifier, defaulting to ALL, "){, MUL \#[\<imm\>](#imm__45 "Is the immediate multiplier, in the range 1 to 16, defaulting to 1, encoded in the "imm4" field.")}}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 64; let dn : integer = UInt(Zdn); let pat : bits(5) = pattern; let imm : integer = UInt(imm4) + 1; let unsigned : boolean = FALSE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn&gt;</td>
<td><span id="Zdn__2"></span>
<p>Is the name of the source and destination scalable vector register, encoded in the "Zdn" field.</p></td>
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

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__45"></span>
<p>Is the immediate multiplier, in the range 1 to 16, defaulting to 1, encoded in the "imm4" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); let operand1 : bits(VL) = Z{}(dn); var result : bits(VL); for e = 0 to elements-1 do let op1elt : bits(esize) = operand1\[e\*:esize\]; if unsigned then (result\[e\*:esize\], -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){esize}(UInt(op1elt) + (count \* imm)); else (result\[e\*:esize\], -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){esize}(SInt(op1elt) + (count \* imm)); end; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
