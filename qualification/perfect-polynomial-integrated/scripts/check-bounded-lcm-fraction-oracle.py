"""Independent exact rational-convolution vectors for private bounded-LCM tests."""
from fractions import Fraction as F
a=[F(1,2),F(-2,3),F(0),F(5,7)]
b=[F(-3,5),F(4,9),F(1,2),F(-2,7)]
product=[sum((a[i]*b[k-i] for i in range(len(a)) if 0 <= k-i < len(b)),F(0)) for k in range(len(a)+len(b)-1)]
expected=[F(-3,10),F(28,45),F(-5,108),F(-19,21),F(32,63),F(5,14),F(-10,49)]
assert product == expected, (product,expected)
print("PASS independent Python fractions.Fraction 4x4 convolution:", ",".join(map(str,product)))
