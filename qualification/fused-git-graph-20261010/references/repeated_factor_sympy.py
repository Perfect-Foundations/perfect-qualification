"""Independent SymPy 1.14.0 ZZ[x]/QQ[x] oracle for M2 repeated-factor tests.

Run: python references/repeated_factor_sympy.py (requires SymPy 1.14.0).
Not a Cargo build dependency and not used to generate expected values at test time.
"""
import sympy as s

x = s.Symbol("x")
h = x**2 + 3*x + 5
u = x**7 - 3*x**5 + 2*x + 4
v = x**6 + 5*x**2 - x + 1
a = s.Poly(-42*h*h*u, x, domain=s.ZZ)
b = s.Poly(30*h*h*v, x, domain=s.ZZ)
def asc(p):
    return [int(z) for z in reversed(p.all_coeffs())]
assert s.gcd(s.Poly(u, x), s.Poly(v, x)) == s.Poly(1, x)
expected_a = [-4200,-7140,-5712,-2604,-672,3066,3780,1344,-504,-672,-252,-42]
expected_b = [750,150,3420,4110,2700,870,900,900,570,180,30]
expected_g = [150,180,114,36,6]
assert asc(a) == expected_a
assert asc(b) == expected_b
assert asc(s.gcd(a,b)) == expected_g
assert asc(s.Poly(h*h, x)) == [25,30,19,6,1]
assert asc(a.exquo(s.Poly(h*h,x))) == [-168,-84,0,0,0,126,0,-42]
print(f"SYMPY_REFERENCE_PASS version={s.__version__} gcd_coeffs={expected_g}")
