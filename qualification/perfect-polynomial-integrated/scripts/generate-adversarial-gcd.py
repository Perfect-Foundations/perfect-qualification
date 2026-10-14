"""Deterministic, independent SymPy ZZ[x]/GF(p)[x] adversarial GCD oracle."""
from pathlib import Path
from sympy import Poly, ZZ, symbols
from random import Random
p=2147483647
x=symbols("x")
rng=Random(0xA11CE2026)
cases=[]
def Z(v):
    return Poly(v,x,domain=ZZ)
def add(name,a,b):
    a=Z(a);b=Z(b)
    assert (not a.is_zero) or (not b.is_zero),name
    cases.append((name,a,b))
for degree in (7,8,9,31,32,33):
    add(f"boundary_monomial_const_d{degree}",x**degree+(1<<63)+1,x**degree+(1<<63)+2)
    add(f"boundary_monomial_noconst_d{degree}",x**degree,x**degree+p*x**3)
    add(f"boundary_lead_p_d{degree}",p*x**degree+1,p*x**degree+2)
for bits in (63,64,65,126,127):
    v=(1<<bits)-1
    add(f"width_{bits}_coprime",x**8+v,x**8+v+p*x)
    add(f"width_{bits}_shared", (x+1)*(x**8+v),(x+1)*(x**8+v-1))
for i in range(12):
    da=8+i%4;db=8+(i*3)%5
    a=sum(((-1)**j)*(1+rng.randrange(90000))*x**j for j in range(da+1))
    b=sum(((-1)**(j+i))*(1+rng.randrange(90000))*x**j for j in range(db+1))
    add(f"dense_{i}",a,b)
for i,f in enumerate([x+1,x-1,x, x**2+1,x**2+x+1,(2*x+3),x**3+2*x+1,(x+1)**2,(x**2+1)**2,x**3, 3*x**2+2*x+7]):
    a=x**(8+i%3)+11+i
    b=x**(9+i%3)+17+i
    add(f"shared_struct_{i}",f*a,f*b)
for i in range(8):
    a=x**(9+i%3)+rng.randint(1,10**6)
    b=x**(9+i%3)+rng.randint(1,10**6)
    # congruent mod p, nonconstant modular gcd but exact gcd 1
    add(f"unlucky_{i}",a,b+p*x**(1+i%4))
for i in range(8):
    a=x**8+2**65+i*13+1
    b=x**8+2**65+i*13+2
    mult_a = (-6 if i%2 else 9)
    mult_b = (15 if i%2 else -12)
    add(f"signed_content_{i}",mult_a*a,mult_b*b)
for i in range(6):
    a=x**(7+i%4)+3+i
    add(f"repeated_{i}",(x*x+1)**2*a,(x*x+1)**2*(a+2))
add("zero_left",0,x**8+1)
add("constant_left",7,x**8+1)
add("same_poly",(x+1)**3,(x+1)**3)
add("neg_same",-3*(x*x+1),6*(x*x+1))
add("unlucky_prime_nonmonomial",x**8+x+1,x**8+(p+1)*x+1)
add("large_common_nonmonic",(3*x+2)*(x**8+1),(3*x+2)*(x**9+7))
# Encode exact Z[x] GCD as coefficient list; independent Fp[x] gcd degree.
def coeff(q):
    if q.is_zero:return "_"
    return ",".join(str(int(v)) for v in reversed(q.all_coeffs()))
out=["name\ta\tb\tgcd_z\tmod_admissible\tmod_degree\tdegrees"]
for name,a,b in cases:
    g=a.gcd(b)
    fa=Poly(a.as_expr(),x,modulus=p)
    fb=Poly(b.as_expr(),x,modulus=p)
    admissible=int(not a.is_zero and not b.is_zero and (int(a.LC())%p)!=0 and (int(b.LC())%p)!=0)
    gd=fa.gcd(fb).degree() if admissible else -1
    assert not(admissible and gd==0 and g.degree()>0),name
    for c in [a,b,g]:
        assert all(-(1<<127)<=int(v)<(1<<127) for v in c.all_coeffs()),name
    out.append("\t".join([name,coeff(a),coeff(b),coeff(g),str(admissible),str(gd),f"{a.degree()},{b.degree()}"]))
path=Path(__file__).parent.parent / "vectors" / "adversarial-gcd-v1.tsv"
path.write_text("\n".join(out)+"\n",encoding="utf-8")
print(f"PASS {len(cases)} independently calculated structured ZZ[x]/GF(p)[x] oracle pairs, certified positives",sum(int(row.split("\t")[5])==0 for row in out[1:]))
