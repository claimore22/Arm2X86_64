## SQINCW (scalar)

Signed saturating increment scalar by multiple of 32-bit predicate constraint element count

This instruction determines the number of active 32-bit elements implied by the named predicate constraint, multiplies that by an immediate in the range 1 to 16 inclusive, and then uses the result to increment the scalar destination. The result is saturated to the source general-purpose register's signed integer range. A 32-bit saturated result is then sign-extended to 64 bits.

The named predicate constraint limits the number of active elements in a single predicate to:

- A fixed number (VL1 to VL256)
- The largest power of two (POW2)
- The largest multiple of three or four (MUL3 or MUL4)
- All available, implicitly a multiple of two (ALL).

Unspecified or out of range constraint encodings generate an empty predicate or zero element count rather than Undefined Instruction exception.

It has encodings from 2 classes: [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

### <span id="iclass_32_bit"></span>32-bit<span style="font-size:smaller;">\
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
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="4" class="lr">imm4</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">pattern</td>
<td colspan="5" class="lr">Rdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td class="droppedname">sf</td>
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

<span id="sqincw_r_rs_sx"></span>

SQINCW [\<Xdn\>](#Xdn "Is the 64-bit name of the source and destination general-purpose register, encoded in the "Rdn" field."), [\<Wdn\>](#Wdn "Is the 32-bit name of the source and destination general-purpose register, encoded in the "Rdn" field."){, [\<pattern\>](#pattern "Is the optional pattern specifier, defaulting to ALL, "){, MUL \#[\<imm\>](#imm__45 "Is the immediate multiplier, in the range 1 to 16, defaulting to 1, encoded in the "imm4" field.")}}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let dn : integer = UInt(Rdn); let pat : bits(5) = pattern; let imm : integer = UInt(imm4) + 1; let unsigned : boolean = FALSE; let ssize : integer{} = 32;

### <span id="iclass_64_bit"></span>64-bit<span style="font-size:smaller;">\
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
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="4" class="lr">imm4</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">pattern</td>
<td colspan="5" class="lr">Rdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td class="droppedname">sf</td>
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

<span id="sqincw_r_rs_x"></span>

SQINCW [\<Xdn\>](#Xdn "Is the 64-bit name of the source and destination general-purpose register, encoded in the "Rdn" field."){, [\<pattern\>](#pattern "Is the optional pattern specifier, defaulting to ALL, "){, MUL \#[\<imm\>](#imm__45 "Is the immediate multiplier, in the range 1 to 16, defaulting to 1, encoded in the "imm4" field.")}}

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let dn : integer = UInt(Rdn); let pat : bits(5) = pattern; let imm : integer = UInt(imm4) + 1; let unsigned : boolean = FALSE; let ssize : integer{} = 64;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xdn&gt;</td>
<td><span id="Xdn"></span>
<p>Is the 64-bit name of the source and destination general-purpose register, encoded in the "Rdn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wdn&gt;</td>
<td><span id="Wdn"></span>
<p>Is the 32-bit name of the source and destination general-purpose register, encoded in the "Rdn" field.</p></td>
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

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let count : integer = [DecodePredCount](shared_pseudocode.html#func_DecodePredCount_2)(pat, esize); let operand1 : bits(ssize) = X{}(dn); var result : bits(ssize); if unsigned then (result, -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){ssize}(UInt(operand1) + (count \* imm)); else (result, -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){ssize}(SInt(operand1) + (count \* imm)); end; [X](shared_pseudocode.html#accessor_X_2){64}(dn) = Extend{64}(result, unsigned);

</div>
