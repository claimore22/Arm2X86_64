## EXT

Extract vector from pair of vectors

This instruction copies the indexed byte up to the last byte of the first source vector to the bottom of the result vector, then fills the remainder of the result starting from the first byte of the second source vector. The result is destructively placed in the destination and first source vector, or constructively placed in the destination vector. This instruction is unpredicated.

An index that is greater than or equal to the vector length in bytes is treated as zero, resulting in the first source vector being copied to the result unchanged.

It has encodings from 2 classes: [Constructive](#iclass_constructive) and [Destructive](#iclass_destructive)

### <span id="iclass_constructive"></span>Constructive<span style="font-size:smaller;">\
(FEAT_SVE2 \|\| FEAT_SME) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">imm8h</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">imm8l</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ext_z_zi_con"></span>

EXT [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").B, { [\<Zn1\>](#Zn1__5 "Is the name of the first scalable vector register of the source multi-vector group, encoded in the "Zn" field.").B, [\<Zn2\>](#Zn2__4 "Is the name of the second scalable vector register of the source multi-vector group, encoded in the "Zn" field.").B }, \#[\<imm\>](#imm__49 "Is the unsigned immediate operand, in the range 0 to 255, encoded in the "imm8h:imm8l" fields.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let dst : integer = UInt(Zd); let s1 : integer = UInt(Zn); let s2 : integer = (s1 + 1) MOD 32; let position : integer{} = UInt(imm8h::imm8l) \* 8;

### <span id="iclass_destructive"></span>Destructive<span style="font-size:smaller;">\
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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">imm8h</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="3" class="lr">imm8l</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="5" class="lr">Zdn</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="ext_z_zi_des"></span>

EXT [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").B, [\<Zdn\>](#Zdn "Is the name of the first source and destination scalable vector register, encoded in the "Zdn" field.").B, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").B, \#[\<imm\>](#imm__49 "Is the unsigned immediate operand, in the range 0 to 255, encoded in the "imm8h:imm8l" fields.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8; let dst : integer = UInt(Zdn); let s1 : integer = dst; let s2 : integer = UInt(Zm); let position : integer{} = UInt(imm8h::imm8l) \* 8;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__5"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__4"></span>
<p>Is the name of the second scalable vector register of the source multi-vector group, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__49"></span>
<p>Is the unsigned immediate operand, in the range 0 to 255, encoded in the "imm8h:imm8l" fields.</p></td>
</tr>
</tbody>
</table>

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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = Z{}(s1); let operand2 : bits(VL) = Z{}(s2); var result : bits(VL); let concat : bits(VL\*2) = operand2 :: operand1; if position \>= VL then result = concat\[VL-1:0\]; else result = concat\[(position+VL)-1:position\]; end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(dst) = result;

</div>

### Operational information

For the "Constructive" variant:

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.

For the "Destructive" variant:

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.

The destructive variant of this instruction might be immediately preceded in program order by a <span class="asm-code">MOVPRFX</span> instruction. The <span class="asm-code">MOVPRFX</span> must conform to all of the following requirements, otherwise the behavior of the <span class="asm-code">MOVPRFX</span> and the destructive variant of this instruction is <span class="arm-defined-word">CONSTRAINED UNPREDICTABLE</span>:

- The <span class="asm-code">MOVPRFX</span> must be unpredicated.
- The <span class="asm-code">MOVPRFX</span> must specify the same destination register as this instruction.
- The destination register must not refer to architectural register state referenced by any other source operand register of this instruction.
