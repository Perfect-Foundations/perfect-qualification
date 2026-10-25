"""Independent Wave 16 exact 1024-flow narrow-day currency oracle."""
from collections import defaultdict
from decimal import Decimal, localcontext
expected={
    "EUR": Decimal("-7999999.936"),
    "GBP": Decimal("-8999999.928"),
    "JPY": Decimal("9000000.063"),
    "USD": Decimal("9000000.063"),
}
with localcontext() as context:
    context.prec=80
    totals=defaultdict(lambda:Decimal(0))
    for i in range(1024):
        day=i%29+1
        if day!=15:continue
        currency=("USD","EUR","JPY","GBP")[i%4]
        amount=Decimal("1000000.007") if i%2==0 else Decimal("-999999.992")
        totals[currency]+=amount
    assert dict(totals)==expected,(totals,expected)
    print("W16_EXACT_DECIMAL_1024_NARROW_ORACLE_PASS",sorted(totals.items()))
