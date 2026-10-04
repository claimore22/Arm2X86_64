## DC

Data cache operation

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
<td colspan="4" class="lr">CRm</td>
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
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="DC_SYS_CR_systeminstrs"></span>

DC [\<dc_op\>](#dc_op_option "Is a DC operation name, as listed for the DC system instruction group, "), [\<Xt\>](#XtOrXZR__3 "Is the 64-bit name of the general-purpose source register, encoded in the "Rt" field.")

is equivalent to

[SYS](sys.html#SYS_CR_systeminstrs) \#[\<op1\>](sys.html#op1), C7, [\<Cm\>](sys.html#Cm), \#[\<op2\>](sys.html#op2), [\<Xt\>](sys.html#XtOrXZR__2)

and is the preferred disassembly when <span class="pseudocode">SysOp(op1, '0111', CRm, op2) == Sys_DC</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;dc_op&gt;</td>
<td><span id="dc_op_option"></span>
<p>Is a DC operation name, as listed for the DC system instruction group, encoded in “(op1 :: CRm :: op2)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op1</th>
<th class="bitfield">CRm</th>
<th class="bitfield">op2</th>
<th class="symbol">&lt;dc_op&gt;</th>
<th class="symbol">Architectural Feature</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0110</td>
<td class="bitfield">001</td>
<td class="symbol">IVAC</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC IVAC.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0110</td>
<td class="bitfield">010</td>
<td class="symbol">ISW</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC ISW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0110</td>
<td class="bitfield">011</td>
<td class="symbol">IGVAC</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC IGVAC.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0110</td>
<td class="bitfield">100</td>
<td class="symbol">IGSW</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC IGSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0110</td>
<td class="bitfield">101</td>
<td class="symbol">IGDVAC</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC IGDVAC.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">0110</td>
<td class="bitfield">110</td>
<td class="symbol">IGDSW</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC IGDSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1010</td>
<td class="bitfield">010</td>
<td class="symbol">CSW</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC CSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1010</td>
<td class="bitfield">100</td>
<td class="symbol">CGSW</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CGSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1010</td>
<td class="bitfield">110</td>
<td class="symbol">CGDSW</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CGDSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1110</td>
<td class="bitfield">010</td>
<td class="symbol">CISW</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC CISW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1110</td>
<td class="bitfield">100</td>
<td class="symbol">CIGSW</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CIGSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1110</td>
<td class="bitfield">110</td>
<td class="symbol">CIGDSW</td>
<td class="feature">FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CIGDSW.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1111</td>
<td class="bitfield">001</td>
<td class="symbol">CIVAPS</td>
<td class="feature">FEAT_PoPS</td>
<td class="description"><p>For more information, see DC CIVAPS.</p></td>
</tr>
<tr>
<td class="bitfield">000</td>
<td class="bitfield">1111</td>
<td class="bitfield">101</td>
<td class="symbol">CIGDVAPS</td>
<td class="feature">FEAT_PoPS &amp;&amp; FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CIGDVAPS.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">0100</td>
<td class="bitfield">001</td>
<td class="symbol">ZVA</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC ZVA.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">0100</td>
<td class="bitfield">011</td>
<td class="symbol">GVA</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC GVA.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">0100</td>
<td class="bitfield">100</td>
<td class="symbol">GZVA</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC GZVA.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1010</td>
<td class="bitfield">001</td>
<td class="symbol">CVAC</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC CVAC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1010</td>
<td class="bitfield">011</td>
<td class="symbol">CGVAC</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGVAC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1010</td>
<td class="bitfield">101</td>
<td class="symbol">CGDVAC</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGDVAC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1011</td>
<td class="bitfield">000</td>
<td class="symbol">CVAOC</td>
<td class="feature">FEAT_OCCMO</td>
<td class="description"><p>For more information, see DC CVAOC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1011</td>
<td class="bitfield">001</td>
<td class="symbol">CVAU</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC CVAU.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1011</td>
<td class="bitfield">111</td>
<td class="symbol">CGDVAOC</td>
<td class="feature">FEAT_OCCMO &amp;&amp; FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGDVAOC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1100</td>
<td class="bitfield">001</td>
<td class="symbol">CVAP</td>
<td class="feature">FEAT_DPB</td>
<td class="description"><p>For more information, see DC CVAP.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1100</td>
<td class="bitfield">011</td>
<td class="symbol">CGVAP</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGVAP.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1100</td>
<td class="bitfield">101</td>
<td class="symbol">CGDVAP</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGDVAP.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1101</td>
<td class="bitfield">001</td>
<td class="symbol">CVADP</td>
<td class="feature">FEAT_DPB2</td>
<td class="description"><p>For more information, see DC CVADP.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1101</td>
<td class="bitfield">011</td>
<td class="symbol">CGVADP</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGVADP.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1101</td>
<td class="bitfield">101</td>
<td class="symbol">CGDVADP</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CGDVADP.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1110</td>
<td class="bitfield">001</td>
<td class="symbol">CIVAC</td>
<td class="feature">-</td>
<td class="description"><p>For more information, see DC CIVAC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1110</td>
<td class="bitfield">011</td>
<td class="symbol">CIGVAC</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CIGVAC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1110</td>
<td class="bitfield">101</td>
<td class="symbol">CIGDVAC</td>
<td class="feature">FEAT_MTE</td>
<td class="description"><p>For more information, see DC CIGDVAC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1111</td>
<td class="bitfield">000</td>
<td class="symbol">CIVAOC</td>
<td class="feature">FEAT_OCCMO</td>
<td class="description"><p>For more information, see DC CIVAOC.</p></td>
</tr>
<tr>
<td class="bitfield">011</td>
<td class="bitfield">1111</td>
<td class="bitfield">111</td>
<td class="symbol">CIGDVAOC</td>
<td class="feature">FEAT_OCCMO &amp;&amp; FEAT_MTE</td>
<td class="description"><p>For more information, see DC CIGDVAOC.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1110</td>
<td class="bitfield">000</td>
<td class="symbol">CIPAE</td>
<td class="feature">FEAT_MEC</td>
<td class="description"><p>For more information, see DC CIPAE.</p></td>
</tr>
<tr>
<td class="bitfield">100</td>
<td class="bitfield">1110</td>
<td class="bitfield">111</td>
<td class="symbol">CIGDPAE</td>
<td class="feature">FEAT_MEC &amp;&amp; FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CIGDPAE.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1110</td>
<td class="bitfield">001</td>
<td class="symbol">CIPAPA</td>
<td class="feature">FEAT_RME</td>
<td class="description"><p>For more information, see DC CIPAPA.</p></td>
</tr>
<tr>
<td class="bitfield">110</td>
<td class="bitfield">1110</td>
<td class="bitfield">101</td>
<td class="symbol">CIGDPAPA</td>
<td class="feature">FEAT_RME &amp;&amp; FEAT_MTE2</td>
<td class="description"><p>For more information, see DC CIGDPAPA.</p></td>
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
