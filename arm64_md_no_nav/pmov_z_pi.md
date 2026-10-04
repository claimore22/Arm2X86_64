## PMOV (to vector)

Move predicate to vector

This instruction copies the source SVE predicate register elements into the destination vector register as a packed bitmap with one bit per predicate element, where bit value 0b1 represents a TRUE predicate element, and bit value 0b0 represents a FALSE predicate element.

Because the number of bits in an SVE predicate element scales with the vector element size, the behavior varies according to the specified element size.

- When the predicate element specifier is B, every bit in the predicate register is copied to the least-significant VL/8 bits of the destination vector register. The portion index, if specified, must be 0.
- When the predicate element specifier is H, every second bit in the predicate register is copied to the indexed block of VL/16 bits in the destination vector register, where the portion index is in the range 0 to 1, inclusive.
- When the predicate element specifier is S, every fourth bit in the predicate register is copied to the indexed block of VL/32 bits in the destination vector register, where the portion index is in the range 0 to 3, inclusive.
- When the predicate element specifier is D, every eighth bit in the predicate register is copied to the indexed block of VL/64 bits in the destination vector register, where the portion index is in the range 0 to 7, inclusive.

The portion index is optional, defaulting to 0 if omitted. When the index is zero, the instruction writes zeroes to the most significant VL-(VL/esize) bits of the destination vector register. When a non-zero index is specified, the packed bitmap is inserted into the destination vector register, and the unindexed blocks remain unchanged.

It has encodings from 4 classes: [Byte](#iclass_byte) , [Doubleword](#iclass_doubleword) , [Halfword](#iclass_halfword) and [Word](#iclass_word)

### <span id="iclass_byte"></span>Byte<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opc2</td>
<td></td>
<td colspan="6"></td>
<td></td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_z_pi_b"></span>

PMOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field."), [\<Pn\>](#Pn__3 "Is the name of the source scalable predicate register, encoded in the "Pn" field.").B

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Pn); let d : integer = UInt(Zd); let esize : integer{} = 8; let imm : integer = 0;

### <span id="iclass_doubleword"></span>Doubleword<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">i3h</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="2" class="lr">i3l</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_z_pi_d"></span>

PMOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field."){\[[\<imm\>](#imm__83 "For the "Doubleword" variant: is the optional portion index, in the range 0 to 7, defaulting to 0, encoded in the "i3h:i3l" fields.")\]}, [\<Pn\>](#Pn__3 "Is the name of the source scalable predicate register, encoded in the "Pn" field.").D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Pn); let d : integer = UInt(Zd); let esize : integer{} = 64; let imm : integer = UInt(i3h::i3l);

### <span id="iclass_halfword"></span>Halfword<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">i1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td colspan="6"></td>
<td></td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_z_pi_h"></span>

PMOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field."){\[[\<imm\>](#imm__84 "For the "Halfword" variant: is the optional portion index, in the range 0 to 1, defaulting to 0, encoded in the "i1" field.")\]}, [\<Pn\>](#Pn__3 "Is the name of the source scalable predicate register, encoded in the "Pn" field.").H

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Pn); let d : integer = UInt(Zd); let esize : integer{} = 16; let imm : integer = UInt(i1);

### <span id="iclass_word"></span>Word<span style="font-size:smaller;">\
(FEAT_SVE2p1 \|\| FEAT_SME2p1) </span>

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
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="2" class="lr">i2</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">opc</td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td></td>
<td colspan="6"></td>
<td></td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="pmov_z_pi_s"></span>

PMOV [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field."){\[[\<imm\>](#imm__85 "For the "Word" variant: is the optional portion index, in the range 0 to 3, defaulting to 0, encoded in the "i2" field.")\]}, [\<Pn\>](#Pn__3 "Is the name of the source scalable predicate register, encoded in the "Pn" field.").S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE2p1) && !IsFeatureImplemented(FEAT_SME2p1) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Pn); let d : integer = UInt(Zd); let esize : integer{} = 32; let imm : integer = UInt(i2);

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
<td>&lt;Pn&gt;</td>
<td><span id="Pn__3"></span>
<p>Is the name of the source scalable predicate register, encoded in the "Pn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__83"></span>
<p>For the "Doubleword" variant: is the optional portion index, in the range 0 to 7, defaulting to 0, encoded in the "i3h:i3l" fields.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__84"></span>
<p>For the "Halfword" variant: is the optional portion index, in the range 0 to 1, defaulting to 0, encoded in the "i1" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__85"></span>
<p>For the "Word" variant: is the optional portion index, in the range 0 to 3, defaulting to 0, encoded in the "i2" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand : bits(PL) = P{}(n); var result : bits(VL); if imm == 0 then result = Zeros{VL}; else result = [Z](shared_pseudocode.html#accessor_Z_2){VL}(d); end; for e = 0 to elements-1 do result\[(elements \* imm) + e\] = [PredicateElement](shared_pseudocode.html#func_PredicateElement_4){PL}(operand, e, esize); end; [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
