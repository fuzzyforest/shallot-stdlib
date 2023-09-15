#![feature(macro_metavar_expr)]
use shallot::*;
use shallot_hashmap::HashMap;
use shallot_strings::LispString;

create_layer!(
   over shallot_regex shallot_strings shallot_hashmap
   | atoms
   | builtins
);
