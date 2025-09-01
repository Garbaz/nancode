# NaNcode

This is a small demo to show that one can "hide" secret bytes in IEEE 754 floating point NaN values.

```
Original message:
  This is a secret message hidden in f64 NaN values.

Encoded message:
  [NaN, NaN, NaN, NaN, NaN, NaN, NaN, NaN, NaN]

Encoded message as bits:
  0111111111110000011010010010000001110011011010010110100001010100
  0111111111110000011001010111001100100000011000010010000001110011
  0111111111110000011011010010000001110100011001010111001001100011
  0111111111110000011001010110011101100001011100110111001101100101
  0111111111110000011001010110010001100100011010010110100000100000
  0111111111110000011001100010000001101110011010010010000001101110
  0111111111110000010011100110000101001110001000000011010000110110
  0111111111110000011001010111010101101100011000010111011000100000
  0111111111110000000000000000000000000000000000000010111001110011

Decoded message:
  This is a secret message hidden in f64 NaN values.
```

The reason this works is that a IEEE 754 float with an exponent of all 1s is understood to be NaN, irrespective of the fraction bits (except for the special case of the fraction bits being all 0s, which is positive/negative infinity, depending on the sign).

Therefore we can "hide" arbitrary data in the fraction bits. If inspected by printing , the value is simply `NaN`, 