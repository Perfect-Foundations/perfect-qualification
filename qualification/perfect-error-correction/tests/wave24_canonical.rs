//! Canonical Wave 24 finite-field evaluation and mixed decoding transcript.
//! GF(7) uses integer modular evaluation; GF(16) uses an independent
//! binary-polynomial product (irreducible modulus 0x13).
use perfect_error_correction::{
    ExtensionField, ExtensionLimits, Natural, PrimeField, ReedSolomon,
    RsDecodeOutcome, RsDecodeSession, RsField, RsSymbol,
};
use perfect_number_theory::{classify_u64, PrimalityResult, PrimeModulus};
fn p(n:u64)->PrimeField {
    let PrimalityResult::ProvenPrime(proof)=classify_u64(n) else {panic!("prime");};
    PrimeField::new(PrimeModulus::from_proven(proof).unwrap())
}
fn mul16(mut a:u64,mut b:u64)->u64{
    let mut result=0_u64;
    while b>0 {
        if b&1!=0 {result^=a;}
        b>>=1;
        a<<=1;
        if a&0x10!=0 {a^=0x13;}
    }
    result
}
fn ord(code:&ReedSolomon,v:u64)->RsSymbol {code.field().ordinal(v).unwrap()}
fn run(code:&ReedSolomon,session:&mut RsDecodeSession,case:usize,field:u64){
    let k=code.message_len();
    let coefficients=(0..k).map(|i|((case as u64*11+i as u64*7+3*i as u64*i as u64+5)%field)).collect::<Vec<_>>();
    let message=coefficients.iter().map(|&v|ord(code,v)).collect::<Vec<_>>();
    let encoded=code.encode(&message).unwrap();
    let expected=(0..code.code_len()).map(|x|{
        let mut value=0_u64;
        let mut power=1_u64;
        for &coefficient in &coefficients {
            if field==7 {
                value=(value+coefficient*power)%field;
                power=power*(x as u64)%field;
            } else {
                value^=mul16(coefficient,power);
                power=mul16(power,x as u64);
            }
        }
        value
    }).collect::<Vec<_>>();
    for (&v,actual) in expected.iter().zip(&encoded){
        assert_eq!(actual,&ord(code,v),"case={case}: independent evaluation");
    }
    let error=(case*5)%code.code_len();
    let mut changed=encoded.clone();
    changed[error]=changed[error].add(&code.field().one()).unwrap();
    let erasures=if field==7 {vec![(error+2)%code.code_len()]}
        else {vec![(error+2)%code.code_len(),(error+4)%code.code_len()]};
    for &i in &erasures {changed[i]=changed[i].add(&code.field().one()).unwrap();}
    assert_eq!(session.decode(&changed,&erasures).unwrap(),RsDecodeOutcome::Corrected);
    assert_eq!(session.last_successful().message,message);
    assert_eq!(session.last_successful().codeword,encoded);
    let mut positions=erasures.clone();
    positions.push(error);
    positions.sort_unstable();
    assert_eq!(session.last_successful().corrected_positions,positions);
    let saved=session.last_successful().clone();
    assert!(session.decode(&encoded,&[code.code_len()]).is_err());
    assert_eq!(session.last_successful(),&saved);
    let symbols=expected.iter().map(u64::to_string).collect::<Vec<_>>().join(",");
    let coords=positions.iter().map(usize::to_string).collect::<Vec<_>>().join(",");
    println!("W24|RS|{case:03}|{field}|{symbols}|{coords}");
}
#[test]
fn canonical_wave24_gf7_gf16_mixed_decoder(){
    let code7=ReedSolomon::new(RsField::Prime(p(7)),6,3,4096).unwrap();
    let base=p(2);
    let defining=[1_u64,1,0,0,1].iter()
        .map(|&v|base.element(Natural::from(v)).unwrap()).collect();
    let code16=ReedSolomon::new(
        RsField::Extension(ExtensionField::new(base,defining,ExtensionLimits::default()).unwrap()),
        8,4,4096).unwrap();
    let mut session7=RsDecodeSession::new(code7.clone()).unwrap();
    let mut session16=RsDecodeSession::new(code16.clone()).unwrap();
    for case in 0..128 {
        if case%2==0 {run(&code7,&mut session7,case,7);}
        else {run(&code16,&mut session16,case,16);}
    }
}
