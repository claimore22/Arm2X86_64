## MOV (vector to tile, single)

Move Z vector to ZA tile slice

This instruction operates on individual horizontal or vertical slices within a named ZA tile of the specified element size. The slice number within the tile is selected by the sum of the slice index register and immediate offset, modulo the number of such elements in a vector. The immediate offset is in the range 0 to the number of elements in a 128-bit vector segment minus 1. Inactive elements in the destination slice remain unmodified.

This is an alias of [MOVA (vector to tile, single)](mova_za_p_rz.html). This means:

- The encodings in this description are named to match the encodings of [MOVA (vector to tile, single)](mova_za_p_rz.html).
- The description of [MOVA (vector to tile, single)](mova_za_p_rz.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

It has encodings from 5 classes: [8-bit](#iclass_8_bit) , [16-bit](#iclass_16_bit) , [32-bit](#iclass_32_bit) , [64-bit](#iclass_64_bit) and [128-bit](#iclass_128_bit)

### <span id="iclass_8_bit"></span>8-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">off4</td>
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
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 8-bit variant

<span id="mov_za_p_rz_b_mova_za_p_rz_b"></span>

MOV ZA0[\<HV\>](#HV__7 "Is the horizontal or vertical slice indicator, ").B\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__2 "For the "8-bit" variant: is the slice index offset, in the range 0 to 15, encoded in the "off4" field.")\], [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").B

is equivalent to

[MOVA](mova_za_p_rz.html#mova_za_p_rz_b) ZA0[\<HV\>](mova_za_p_rz.html#HV__7).B\[[\<Ws\>](mova_za_p_rz.html#Ws__3), [\<offs\>](mova_za_p_rz.html#offs__2)\], [\<Pg\>](mova_za_p_rz.html#Pg)/M, [\<Zn\>](mova_za_p_rz.html#Zn).B

and is always the preferred disassembly.

</div>

### <span id="iclass_16_bit"></span>16-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">ZAd</td>
<td colspan="3" class="lr">off3</td>
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
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="mov_za_p_rz_h_mova_za_p_rz_h"></span>

MOV [\<ZAd\>](#ZAd__2 "For the "16-bit" variant: is the name of the ZA tile ZA0-ZA1 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__7 "Is the horizontal or vertical slice indicator, ").H\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__4 "For the "16-bit" variant: is the slice index offset, in the range 0 to 7, encoded in the "off3" field.")\], [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").H

is equivalent to

[MOVA](mova_za_p_rz.html#mova_za_p_rz_h) [\<ZAd\>](mova_za_p_rz.html#ZAd__2)[\<HV\>](mova_za_p_rz.html#HV__7).H\[[\<Ws\>](mova_za_p_rz.html#Ws__3), [\<offs\>](mova_za_p_rz.html#offs__4)\], [\<Pg\>](mova_za_p_rz.html#Pg)/M, [\<Zn\>](mova_za_p_rz.html#Zn).H

and is always the preferred disassembly.

</div>

### <span id="iclass_32_bit"></span>32-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="2" class="lr">ZAd</td>
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
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="mov_za_p_rz_w_mova_za_p_rz_w"></span>

MOV [\<ZAd\>](#ZAd__3 "For the "32-bit" variant: is the name of the ZA tile ZA0-ZA3 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__7 "Is the horizontal or vertical slice indicator, ").S\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__6 "For the "32-bit" variant: is the slice index offset, in the range 0 to 3, encoded in the "off2" field.")\], [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").S

is equivalent to

[MOVA](mova_za_p_rz.html#mova_za_p_rz_w) [\<ZAd\>](mova_za_p_rz.html#ZAd__3)[\<HV\>](mova_za_p_rz.html#HV__7).S\[[\<Ws\>](mova_za_p_rz.html#Ws__3), [\<offs\>](mova_za_p_rz.html#offs__6)\], [\<Pg\>](mova_za_p_rz.html#Pg)/M, [\<Zn\>](mova_za_p_rz.html#Zn).S

and is always the preferred disassembly.

</div>

### <span id="iclass_64_bit"></span>64-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="3" class="lr">ZAd</td>
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
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td colspan="3"></td>
<td></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="mov_za_p_rz_d_mova_za_p_rz_d"></span>

MOV [\<ZAd\>](#ZAd "For the "64-bit" variant: is the name of the ZA tile ZA0-ZA7 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__7 "Is the horizontal or vertical slice indicator, ").D\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__3 "For the "64-bit" variant: is the slice index offset, in the range 0 to 1, encoded in the "o1" field.")\], [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").D

is equivalent to

[MOVA](mova_za_p_rz.html#mova_za_p_rz_d) [\<ZAd\>](mova_za_p_rz.html#ZAd)[\<HV\>](mova_za_p_rz.html#HV__7).D\[[\<Ws\>](mova_za_p_rz.html#Ws__3), [\<offs\>](mova_za_p_rz.html#offs__3)\], [\<Pg\>](mova_za_p_rz.html#Pg)/M, [\<Zn\>](mova_za_p_rz.html#Zn).D

and is always the preferred disassembly.

</div>

### <span id="iclass_128_bit"></span>128-bit<span style="font-size:smaller;">\
(FEAT_SME) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">V</td>
<td colspan="2" class="lr">Rs</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">ZAd</td>
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
<td class="droppedname">Q</td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 128-bit variant

<span id="mov_za_p_rz_q_mova_za_p_rz_q"></span>

MOV [\<ZAd\>](#ZAd__4 "For the "128-bit" variant: is the name of the ZA tile ZA0-ZA15 to be accessed, encoded in the "ZAd" field.")[\<HV\>](#HV__7 "Is the horizontal or vertical slice indicator, ").Q\[[\<Ws\>](#Ws__3 "Is the 32-bit name of the slice index register W12-W15, encoded in the "Rs" field."), [\<offs\>](#offs__5 "For the "128-bit" variant: is the slice index offset, with implicit value 0.")\], [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/M, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").Q

is equivalent to

[MOVA](mova_za_p_rz.html#mova_za_p_rz_q) [\<ZAd\>](mova_za_p_rz.html#ZAd__4)[\<HV\>](mova_za_p_rz.html#HV__7).Q\[[\<Ws\>](mova_za_p_rz.html#Ws__3), [\<offs\>](mova_za_p_rz.html#offs__5)\], [\<Pg\>](mova_za_p_rz.html#Pg)/M, [\<Zn\>](mova_za_p_rz.html#Zn).Q

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
<td><span id="HV__7"></span>
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
<td>&lt;offs&gt;</td>
<td><span id="offs__2"></span>
<p>For the "8-bit" variant: is the slice index offset, in the range 0 to 15, encoded in the "off4" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__4"></span>
<p>For the "16-bit" variant: is the slice index offset, in the range 0 to 7, encoded in the "off3" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__6"></span>
<p>For the "32-bit" variant: is the slice index offset, in the range 0 to 3, encoded in the "off2" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__3"></span>
<p>For the "64-bit" variant: is the slice index offset, in the range 0 to 1, encoded in the "o1" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="offs__5"></span>
<p>For the "128-bit" variant: is the slice index offset, with implicit value 0.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pg&gt;</td>
<td><span id="Pg"></span>
<p>Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
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
<tr>
<td></td>
<td><span id="ZAd__4"></span>
<p>For the "128-bit" variant: is the name of the ZA tile ZA0-ZA15 to be accessed, encoded in the "ZAd" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [MOVA (vector to tile, single)](mova_za_p_rz.html) gives the operational pseudocode for this instruction.

</div>

<div class="alias_ps_section">

### Operational Information

The description of [MOVA (vector to tile, single)](mova_za_p_rz.html) gives the operational information for this instruction.

</div>
