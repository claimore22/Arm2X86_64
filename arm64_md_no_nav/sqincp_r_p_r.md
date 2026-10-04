## SQINCP (scalar)

Signed saturating increment scalar by count of true predicate elements

This instruction determines the number of true elements in the source predicate and then uses the result to increment the scalar destination. The result is saturated to the source general-purpose register's signed integer range. A 32-bit saturated result is then sign-extended to 64 bits.

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pm</td>
<td colspan="5" class="lr">Rdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">D</td>
<td class="droppedname">U</td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">sf</td>
<td class="droppedname">op</td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sqincp_r_p_r_sx"></span>

SQINCP [\<Xdn\>](#Xdn "Is the 64-bit name of the source and destination general-purpose register, encoded in the "Rdn" field."), [\<Pm\>](#Pm__3 "Is the name of the source scalable predicate register, encoded in the "Pm" field.").[\<T\>](#T__31 "Is the size specifier, "), [\<Wdn\>](#Wdn "Is the 32-bit name of the source and destination general-purpose register, encoded in the "Rdn" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let m : integer = UInt(Pm); let dn : integer = UInt(Rdn); let unsigned : boolean = FALSE; let ssize : integer{} = 32;

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
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pm</td>
<td colspan="5" class="lr">Rdn</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">D</td>
<td class="droppedname">U</td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">sf</td>
<td class="droppedname">op</td>
<td colspan="4"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="sqincp_r_p_r_x"></span>

SQINCP [\<Xdn\>](#Xdn "Is the 64-bit name of the source and destination general-purpose register, encoded in the "Rdn" field."), [\<Pm\>](#Pm__3 "Is the name of the source scalable predicate register, encoded in the "Pm" field.").[\<T\>](#T__31 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let m : integer = UInt(Pm); let dn : integer = UInt(Rdn); let unsigned : boolean = FALSE; let ssize : integer{} = 64;

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
<td>&lt;Pm&gt;</td>
<td><span id="Pm__3"></span>
<p>Is the name of the source scalable predicate register, encoded in the "Pm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__31"></span>
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
<td>&lt;Wdn&gt;</td>
<td><span id="Wdn"></span>
<p>Is the 32-bit name of the source and destination general-purpose register, encoded in the "Rdn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let operand1 : bits(ssize) = X{}(dn); let operand2 : bits(PL) = P{}(m); var result : bits(ssize); var count : integer = 0; for e = 0 to elements-1 do if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(operand2, e, esize) then count = count + 1; end; end; if unsigned then (result, -) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){ssize}(UInt(operand1) + count); [X](shared_pseudocode.html#accessor_X_2){64}(dn) = ZeroExtend{64}(result); else (result, -) = [SignedSatQ](shared_pseudocode.html#func_SignedSatQ_2){ssize}(SInt(operand1) + count); [X](shared_pseudocode.html#accessor_X_2){64}(dn) = SignExtend{64}(result); end;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the general-purpose register written by this instruction might be significantly delayed.
