## MOV (vector to tile, four registers)

Move Z four-vector operand to ZA four-slice operand

This instruction operates on a horizontal or vertical ZA four-slice operand within a ZA tile of the specified element size.

The first slice of the four-slice operand is selected by rounding down the sum of the slice index register and the immediate offset to the nearest lower multiple of 4, modulo the number of slices in the tile.

The immediate offset is a multiple of 4 in the range 0 to the number of elements in a 128-bit vector segment minus 4.

This instruction is unpredicated.

This is an alias of [MOVA (vector to tile, four registers)](mova_za4_z.html). This means:

- The encodings in this description are named to match the encodings of [MOVA (vector to tile, four registers)](mova_za4_z.html).
- The description of [MOVA (vector to tile, four registers)](mova_za4_z.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

It has encodings from 4 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) and [64-bit](#iclass_64_bit)

### <span id="iclass_8_bit"></span>8-bit<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">off2</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 8-bit variant

<span id="mov_za4_z_b1_mova_za4_z_b1"></span>

MOV ZA0[\<HV\>](#HV__6 "Is the horizontal or vertical slice indicator, ").B\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs1\>](#offs1__9 "For the "8-bit" variant: is the first slice index offset, encoded as "off2" field times 4."):[\<offs4\>](#offs4__3 "For the "8-bit" variant: is the fourth slice index offset, encoded as "off2" field times 4 plus 3.")\], { [\<Zn1\>](#Zn1__6 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").B-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").B }

is equivalent to

[MOVA](mova_za4_z.html#mova_za4_z_b1) ZA0[\<HV\>](mova_za4_z.html#HV__6).B\[[\<Ws\>](mova_za4_z.html#Ws__3), [\<offs1\>](mova_za4_z.html#offs1__9):[\<offs4\>](mova_za4_z.html#offs4__3)\], { [\<Zn1\>](mova_za4_z.html#Zn1__6).B-[\<Zn4\>](mova_za4_z.html#Zn4__3).B }

and is always the preferred disassembly.

</div>

### <span id="iclass_16_bit"></span>16-bit<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">ZAd</td>
<td class="lr">o1</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="mov_za4_z_h1_mova_za4_z_h1"></span>

MOV [\<ZAd\>](#ZAd__2 "For the "16-bit" variant: is the name of the ZA tile ZA0-ZA1 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__6 "Is the horizontal or vertical slice indicator, ").H\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs1\>](#offs1__10 "For the "16-bit" variant: is the first slice index offset, encoded as "o1" field times 4."):[\<offs4\>](#offs4__5 "For the "16-bit" variant: is the fourth slice index offset, encoded as "o1" field times 4 plus 3.")\], { [\<Zn1\>](#Zn1__6 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").H-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").H }

is equivalent to

[MOVA](mova_za4_z.html#mova_za4_z_h1) [\<ZAd\>](mova_za4_z.html#ZAd__2)[\<HV\>](mova_za4_z.html#HV__6).H\[[\<Ws\>](mova_za4_z.html#Ws__3), [\<offs1\>](mova_za4_z.html#offs1__10):[\<offs4\>](mova_za4_z.html#offs4__5)\], { [\<Zn1\>](mova_za4_z.html#Zn1__6).H-[\<Zn4\>](mova_za4_z.html#Zn4__3).H }

and is always the preferred disassembly.

</div>

### <span id="iclass_32_bit"></span>32-bit<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">ZAd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="mov_za4_z_w1_mova_za4_z_w1"></span>

MOV [\<ZAd\>](#ZAd__3 "For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__6 "Is the horizontal or vertical slice indicator, ").S\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs1\>](#offs1__6 "For the "32-bit" and "64-bit" variants: is the first slice index offset, with implicit value 0."):[\<offs4\>](#offs4__4 "For the "32-bit" and "64-bit" variants: is the fourth slice index offset, with implicit value 3.")\], { [\<Zn1\>](#Zn1__6 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").S-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").S }

is equivalent to

[MOVA](mova_za4_z.html#mova_za4_z_w1) [\<ZAd\>](mova_za4_z.html#ZAd__3)[\<HV\>](mova_za4_z.html#HV__6).S\[[\<Ws\>](mova_za4_z.html#Ws__3), [\<offs1\>](mova_za4_z.html#offs1__6):[\<offs4\>](mova_za4_z.html#offs4__4)\], { [\<Zn1\>](mova_za4_z.html#Zn1__6).S-[\<Zn4\>](mova_za4_z.html#Zn4__3).S }

and is always the preferred disassembly.

</div>

### <span id="iclass_64_bit"></span>64-bit<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">ZAd</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td colspan="3"></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="mov_za4_z_d1_mova_za4_z_d1"></span>

MOV [\<ZAd\>](#ZAd "For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__6 "Is the horizontal or vertical slice indicator, ").D\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs1\>](#offs1__6 "For the "32-bit" and "64-bit" variants: is the first slice index offset, with implicit value 0."):[\<offs4\>](#offs4__4 "For the "32-bit" and "64-bit" variants: is the fourth slice index offset, with implicit value 3.")\], { [\<Zn1\>](#Zn1__6 "Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.").D-[\<Zn4\>](#Zn4__3 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.").D }

is equivalent to

[MOVA](mova_za4_z.html#mova_za4_z_d1) [\<ZAd\>](mova_za4_z.html#ZAd)[\<HV\>](mova_za4_z.html#HV__6).D\[[\<Ws\>](mova_za4_z.html#Ws__3), [\<offs1\>](mova_za4_z.html#offs1__6):[\<offs4\>](mova_za4_z.html#offs4__4)\], { [\<Zn1\>](mova_za4_z.html#Zn1__6).D-[\<Zn4\>](mova_za4_z.html#Zn4__3).D }

and is always the preferred disassembly.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;HV&gt;</td>
<td><span id="HV__6"></span>
<p>Is the horizontal or vertical slice indicator, encoded in “V”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">V</th>
<th class="symbol">&lt;HV&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">V</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ws&gt;</td>
<td><span id="Ws__3"></span>
<p>Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs1&gt;</td>
<td><span id="offs1__9"></span>
<p>For the "8-bit" variant: is the first slice index offset, encoded as "off2" field times 4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs1__10"></span>
<p>For the "16-bit" variant: is the first slice index offset, encoded as "o1" field times 4.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs1__6"></span>
<p>For the "32-bit" and "64-bit" variants: is the first slice index offset, with implicit value 0.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs4&gt;</td>
<td><span id="offs4__3"></span>
<p>For the "8-bit" variant: is the fourth slice index offset, encoded as "off2" field times 4 plus 3.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs4__5"></span>
<p>For the "16-bit" variant: is the fourth slice index offset, encoded as "o1" field times 4 plus 3.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs4__4"></span>
<p>For the "32-bit" and "64-bit" variants: is the fourth slice index offset, with implicit value 3.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__6"></span>
<p>Is the name of the first scalable vector register of the source multi-vector group, encoded as "Zn" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4__3"></span>
<p>Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;ZAd&gt;</td>
<td><span id="ZAd__2"></span>
<p>For the "16-bit" variant: is the name of the ZA tile ZA0-ZA1 to be accessed, encoded in the "ZAd" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="ZAd__3"></span>
<p>For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3 to be accessed, encoded in the "ZAd" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="ZAd"></span>
<p>For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7 to be accessed, encoded in the "ZAd" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [MOVA (vector to tile, four registers)](mova_za4_z.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [MOVA (vector to tile, four registers)](mova_za4_z.html) gives the operational information for this instruction.

</div>
