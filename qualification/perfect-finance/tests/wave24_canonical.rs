//! Canonical exact finance transcript: signed ties, leap day, and stable IDs.
//! Each printed field has an explicit mathematical value or canonical Decimal
//! display; never hash Rust Debug output or iteration order.
use perfect_finance::{
    CashFlow, CurrencyCode, Date, DateBoundary, DateRange, DayCount,
    Decimal, DecimalExponent, DecimalLimits, Money, Month, OwnedCashFlowLedger,
    RoundingMode,
};
fn date(month:Month,day:u8)->Date {
    Date::from_calendar_date(2024,month,day).unwrap()
}
fn flow(d:Date,c:&str,v:i64,lim:DecimalLimits)->CashFlow{
    CashFlow::new(d,Money::new(
        Decimal::parse(&v.to_string(),lim).unwrap(),
        CurrencyCode::new(c).unwrap()))
}
fn decimals(milli:i64)->String {
    let unsigned=milli.unsigned_abs();
    format!("{}{}.{:03}",if milli<0 {"-"} else {""},unsigned/1000,unsigned%1000)
}
fn cent(ticks:i64,negative:bool)->String {
    format!("{}{}.{:02}",if negative{"-"}else{""},ticks/100,ticks%100)
}
fn rational(v:&perfect_finance::Rational)->String{
    let numerator=v.numerator_magnitude().try_to_u128().unwrap().into_value();
    let denominator=v.denominator().try_to_u128().unwrap().into_value();
    format!("{}{numerator}/{denominator}",if v.is_negative(){"-"}else{""})
}
#[test]
fn canonical_wave24_money_daycount_and_ledger(){
    let lim=DecimalLimits::new(12).unwrap();
    let usd=CurrencyCode::new("USD").unwrap();
    let feb28=date(Month::February,28);
    let feb29=date(Month::February,29);
    let mar1=date(Month::March,1);
    let range=DateRange::new(feb28,feb29,
        DateBoundary::Inclusive,DateBoundary::Inclusive).unwrap();
    for case in 0..128_i64 {
        let milli=(case-64)*10+5;
        let amount=Money::new(Decimal::parse(&decimals(milli),lim).unwrap(),usd);
        let even=amount.quantize(DecimalExponent::new(-2),
            RoundingMode::NearestTiesToEven,lim).unwrap();
        let away=amount.quantize(DecimalExponent::new(-2),
            RoundingMode::NearestTiesAway,lim).unwrap();
        let magnitude=milli.unsigned_abs()/10;
        let even_mag=magnitude+u64::from(magnitude%2!=0);
        let away_mag=magnitude+1;
        assert_eq!(even.value().amount(),
            &Decimal::parse(&cent(even_mag as i64,milli<0),lim).unwrap());
        assert_eq!(away.value().amount(),
            &Decimal::parse(&cent(away_mag as i64,milli<0),lim).unwrap());
        let a=case-45;
        let b=23-case%17;
        let c=case%9-4;
        let flows=[flow(feb28,"USD",a,lim),flow(feb29,"USD",b,lim),
            flow(mar1,"EUR",c,lim)];
        let mut ledger=OwnedCashFlowLedger::new(&flows,6,lim).unwrap();
        let first=ledger.ids()[0];
        let next=ledger.append(&[flow(feb29,"EUR",c+2,lim)]).unwrap();
        assert_eq!(next[0].get(),3);
        ledger.replace(first,flow(feb28,"USD",a+1,lim)).unwrap();
        let removed=ledger.remove(next[0]).unwrap();
        assert_eq!(removed.money().amount(),&Decimal::parse(&(c+2).to_string(),lim).unwrap());
        let mut view=ledger.indexed_view().unwrap();
        let net=view.currency_totals(Some(&range)).unwrap();
        assert_eq!(net.len(),1);
        assert_eq!(net[0].currency(),usd);
        assert_eq!(net[0].amount(),&Decimal::parse(&(a+1+b).to_string(),lim).unwrap());
        let d=DayCount::Actual365Fixed.year_fraction(feb28,mar1).unwrap();
        assert_eq!(d,perfect_finance::Rational::from(2_i64)
            .div(&perfect_finance::Rational::from(365_i64)).unwrap());
        println!("W24|FIN|{case:03}|{}|{}|{}|{}|{}|{}",
            even.value().amount(),away.value().amount(),
            net[0].amount(),rational(&d),first.get(),ledger.ids()[2].get());
    }
}
