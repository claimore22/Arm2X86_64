## AT

Address translate

For more information, see <span class="armarm-xref" data-linkend="ARMARM_BABEJJJE">op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions</span>.

This is an alias of [SYS](sys.html). This means:

- The encodings in this description are named to match the encodings of [SYS](sys.html).
- The description of [SYS](sys.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

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
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="3" class="lr">op1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">x</td>
<td colspan="3" class="lr">op2</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4" class="droppedname">CRn</td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="AT_SYS_CR_systeminstrs"></span>

AT [\<at_op\>](#at_op_option "Is an AT operation name, as listed for the AT system instruction group, "), [\<Xt\>](#XtOrXZR__3 "Is the 64-bit name of the general-purpose source register, encoded in the "Rt" field.")

is equivalent to

[SYS](sys.html#SYS_CR_systeminstrs) \#[\<op1\>](sys.html#op1), C7, [\<Cm\>](sys.html#Cm), \#[\<op2\>](sys.html#op2), [\<Xt\>](sys.html#XtOrXZR__2)

and is the preferred disassembly when <span class="pseudocode">SysOp(op1, '0111', CRm, op2) == Sys_AT</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;at_op&gt;</td>
<td><span id="at_op_option"></span>
<p>Is an AT operation name, as listed for the AT system instruction group, encoded in “(op1 :: CRm :: op2)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op1</th>
<th class="bitfield">CRm</th>
<th class="bitfield">op2</th>
<th class="symbol">&lt;at_op&gt;</th>
<th class="symbol">Architectural Feature</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">000</td>
<td class="symbol">S1E1R</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E1R.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">001</td>
<td class="symbol">S1E1W</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E1W.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">010</td>
<td class="symbol">S1E0R</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E0R.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1000</td>
<td class="bitfield">011</td>
<td class="symbol">S1E0W</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E0W.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">000</td>
<td class="symbol">S1E1RP</td>
<td class="feature">FEAT_PAN2</td>
<td class="description"><p>For more information, see AT S1E1RP.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">001</td>
<td class="symbol">S1E1WP</td>
<td class="feature">FEAT_PAN2</td>
<td class="description"><p>For more information, see AT S1E1WP.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1001</td>
<td class="bitfield">010</td>
<td class="symbol">S1E1A</td>
<td class="feature">FEAT_ATS1A</td>
<td class="description"><p>For more information, see AT S1E1A.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">000</td>
<td class="symbol">S1E2R</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E2R.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">001</td>
<td class="symbol">S1E2W</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E2W.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">100</td>
<td class="symbol">S12E1R</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S12E1R.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">101</td>
<td class="symbol">S12E1W</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S12E1W.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">110</td>
<td class="symbol">S12E0R</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S12E0R.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1000</td>
<td class="bitfield">111</td>
<td class="symbol">S12E0W</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S12E0W.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1001</td>
<td class="bitfield">010</td>
<td class="symbol">S1E2A</td>
<td class="feature">FEAT_ATS1A</td>
<td class="description"><p>For more information, see AT S1E2A.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">000</td>
<td class="symbol">S1E3R</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E3R.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1000</td>
<td class="bitfield">001</td>
<td class="symbol">S1E3W</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see AT S1E3W.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1001</td>
<td class="bitfield">010</td>
<td class="symbol">S1E3A</td>
<td class="feature">FEAT_ATS1A</td>
<td class="description"><p>For more information, see AT S1E3A.</p></td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR__3"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.

</div>
