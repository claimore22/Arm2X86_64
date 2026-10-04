## FTMAD

Floating-point trigonometric multiply-add coefficient

This instruction multiplies each element of the first source vector by the absolute value of the corresponding element of the second source vector and performs a fused addition of each product with a value obtained from a table of hard-wired coefficients, and destructively places the results in the first source vector. This instruction is unpredicated.

The coefficients are selected by a combination of the sign bit in the second source element and an immediate index in the range 0 to 7.

<span class="asm-code">FTMAD</span> can be combined with <span class="asm-code">FTSMUL</span> and <span class="asm-code">FTSSEL</span> to calculate values for sin(x) and cos(x). For more information, see <span class="asm-code">FTSMUL</span>. The coefficients are intended to provide accurate results for <span class="asm-code">FTSMUL</span> inputs in the range -π/4 \< x ≤ π/4.

For double-precision operations, the coefficients are:

| element2\<63\> | Index | Hexadecimal         | Approximate value |
|----------------|-------|---------------------|-------------------|
| 0              | 0     | 3ff0 0000 0000 0000 | 1                 |
| 0              | 1     | bfc5 5555 5555 5543 | -1/3!             |
| 0              | 2     | 3f81 1111 1110 f30c | 1/5!              |
| 0              | 3     | bf2a 01a0 19b9 2fc6 | -1/7!             |
| 0              | 4     | 3ec7 1de3 51f3 d22b | 1/9!              |
| 0              | 5     | be5a e5e2 b60f 7b91 | -1/11!            |
| 0              | 6     | 3de5 d840 8868 552f | 1/13!             |
| 0              | 7     | 0000 0000 0000 0000 | 0                 |
| 1              | 0     | 3ff0 0000 0000 0000 | 1                 |
| 1              | 1     | bfe0 0000 0000 0000 | -1/2!             |
| 1              | 2     | 3fa5 5555 5555 5536 | 1/4!              |
| 1              | 3     | bf56 c16c 16c1 3a0b | -1/6!             |
| 1              | 4     | 3efa 01a0 19b1 e8d8 | 1/8!              |
| 1              | 5     | be92 7e4f 7282 f468 | -1/10!            |
| 1              | 6     | 3e21 ee96 d264 1b13 | 1/12!             |
| 1              | 7     | bda8 f763 80fb b401 | -1/14!            |

For single-precision operations, the coefficients are:

| element2\<63\> | Index | Hexadecimal | Approximate value |
|----------------|-------|-------------|-------------------|
| 0              | 0     | 3f80 0000   | 1                 |
| 0              | 1     | be2a aaab   | -1/3!             |
| 0              | 2     | 3c08 8886   | 1/5!              |
| 0              | 3     | b950 08b9   | -1/7!             |
| 0              | 4     | 3636 9d6d   | 1/9!              |
| 0              | 5     | 0000 0000   | 0                 |
| 0              | 6     | 0000 0000   | 0                 |
| 0              | 7     | 0000 0000   | 0                 |
| 1              | 0     | 3f80 0000   | 1                 |
| 1              | 1     | bf00 0000   | -1/2!             |
| 1              | 2     | 3d2a aaa6   | 1/4!              |
| 1              | 3     | bab6 0705   | -1/6!             |
| 1              | 4     | 37cd 37cc   | 1/8!              |
| 1              | 5     | 0000 0000   | 0                 |
| 1              | 6     | 0000 0000   | 0                 |
| 1              | 7     | 0000 0000   | 0                 |

For half-precision operations, the coefficients are:

| element2\<63\> | Index | Hexadecimal | Approximate value |
|----------------|-------|-------------|-------------------|
| 0              | 0     | 3c00        | 1                 |
| 0              | 1     | b155        | -1/3!             |
| 0              | 2     | 2030        | 1/5!              |
| 0              | 3     | 0000        | 0                 |
| 0              | 4     | 0000        | 0                 |
| 0              | 5     | 0000        | 0                 |
| 0              | 6     | 0000        | 0                 |
| 0              | 7     | 0000        | 0                 |
| 1              | 0     | 3c00        | 1                 |
| 1              | 1     | b800        | -1/2!             |
| 1              | 2     | 293a        | 1/4!              |
| 1              | 3     | 0000        | 0                 |
| 1              | 4     | 0000        | 0                 |
| 1              | 5     | 0000        | 0                 |
| 1              | 6     | 0000        | 0                 |
| 1              | 7     | 0000        | 0                 |

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
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="3" class="lr">imm3</td>
<td class="l">1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ftmad_z_zzi_"></span>

FTMAD [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__53 "Is the size specifier, "), [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").[\<T\>](#T__53 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__53 "Is the size specifier, "), \#[\<imm\>](#imm__57 "Is the unsigned immediate operand, in the range 0 to 7, encoded in the "imm3" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size == '00' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let dn : integer = UInt(Zdn); let m : integer = UInt(Zm); let imm : integer = UInt(imm3);

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
<td>&lt;T&gt;</td>
<td><span id="T__53"></span>
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
<td class="symbol">RESERVED</td>
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
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__57"></span>
<p>Is the unsigned immediate operand, in the range 0 to 7, encoded in the "imm3" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(dn); let operand2 : bits(VL) = Z{}(m); var result : bits(VL); for e = 0 to elements-1 do let element1 : bits(esize) = operand1\[e\*:esize\]; let element2 : bits(esize) = operand2\[e\*:esize\]; result\[e\*:esize\] = [FPTrigMAdd](shared_pseudocode.html#func_FPTrigMAdd_5){esize}(imm, element1, element2, FPCR()); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn) = result;

</div>

### Operational information

This instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
