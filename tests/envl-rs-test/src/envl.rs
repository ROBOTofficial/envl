# [deny (clippy :: all)] # [derive (Debug , Clone)] # [rustfmt :: skip] pub struct Env { pub a : String , pub c : bool , pub b : i64 , pub d : Vec < i64 > , } # [rustfmt :: skip] pub fn envl () -> Env { Env { a : String :: from ("123") , b : 123 , c : true , d : Vec :: from ([123 , 456 ,]) , } }

