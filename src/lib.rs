#![feature(macro_metavar_expr)]
use shallot::*;
use shallot_hashmap::HashMap;
use shallot_strings::LispString;

create_layer!(
   over shallot_hashmap, shallot_strings
   | atoms
   | builtins
);
