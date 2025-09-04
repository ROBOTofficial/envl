# [derive (Debug , Clone)] # [rustfmt :: skip] pub struct Env { pub b : i64 , pub c : bool , pub a : String , pub d : Vec < i64 > , } # [rustfmt :: skip] pub fn envl () -> Env { Env { c : true , d : Vec :: from ([123 , 456 ,]) , a : String :: from ("123") , b : 123 , } }

