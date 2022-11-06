pub const DOT_STRING_EMPTY_NET: &str = "digraph petrinet {\n}\n";

pub const DOT_STRING_ONLY_EMPTY_PLACES_NET: &str = "\
digraph petrinet {
    P1 [shape=\"circle\" xlabel=\"P1\" label=\"\"];
    P2 [shape=\"circle\" xlabel=\"P2\" label=\"\"];
    P3 [shape=\"circle\" xlabel=\"P3\" label=\"\"];
    P4 [shape=\"circle\" xlabel=\"P4\" label=\"\"];
    P5 [shape=\"circle\" xlabel=\"P5\" label=\"\"];
}
";

pub const DOT_STRING_MARKED_PLACES_NET: &str = "\
digraph petrinet {
    P1 [shape=\"circle\" xlabel=\"P1\" label=\"•••••\"];
    P2 [shape=\"circle\" xlabel=\"P2\" label=\"6\"];
    P3 [shape=\"circle\" xlabel=\"P3\" label=\"•••\"];
    P4 [shape=\"circle\" xlabel=\"P4\" label=\"••\"];
    P5 [shape=\"circle\" xlabel=\"P5\" label=\"•\"];
}
";

pub const DOT_STRING_ONLY_EMPTY_TRANSITIONS_NET: &str = "\
digraph petrinet {
    T1 [shape=\"box\" xlabel=\"T1\" label=\"\"];
    T2 [shape=\"box\" xlabel=\"T2\" label=\"\"];
    T3 [shape=\"box\" xlabel=\"T3\" label=\"\"];
    T4 [shape=\"box\" xlabel=\"T4\" label=\"\"];
    T5 [shape=\"box\" xlabel=\"T5\" label=\"\"];
}
";

pub const DOT_STRING_NET_WITH_CHAIN_TOPOLOPY: &str = "\
digraph petrinet {
    P1 [shape=\"circle\" xlabel=\"P1\" label=\"\"];
    P2 [shape=\"circle\" xlabel=\"P2\" label=\"\"];
    P3 [shape=\"circle\" xlabel=\"P3\" label=\"\"];
    T1 [shape=\"box\" xlabel=\"T1\" label=\"\"];
    T2 [shape=\"box\" xlabel=\"T2\" label=\"\"];
    P1 -> T1;
    P2 -> T2;
    T1 -> P2;
    T2 -> P3;
}
";

pub const DOT_STRING_NET_WITH_LOOP_TOPOLOGY: &str = "\
digraph petrinet {
    P1 [shape=\"circle\" xlabel=\"P1\" label=\"\"];
    T1 [shape=\"box\" xlabel=\"T1\" label=\"\"];
    P1 -> T1;
    T1 -> P1;
}
";

pub const LOLA_STRING_ONLY_EMPTY_PLACES_NET: &str = "\
PLACE
    P1,
    P2,
    P3,
    P4,
    P5;

MARKING
";

pub const LOLA_STRING_MARKED_PLACES_NET: &str = "\
PLACE
    P1,
    P2,
    P3,
    P4,
    P5;

MARKING
    P1 : 5,
    P2 : 6,
    P3 : 3,
    P4 : 2,
    P5 : 1;

";

pub const LOLA_STRING_ONLY_EMPTY_TRANSITIONS_NET: &str = "\
TRANSITION T1
TRANSITION T2
TRANSITION T3
TRANSITION T4
TRANSITION T5
";

pub const LOLA_STRING_NET_WITH_CHAIN_TOPOLOPY: &str = "\
PLACE
    P1,
    P2,
    P3;

MARKING
TRANSITION T1
  CONSUME
    P1 : 1;
  PRODUCE
    P2 : 1;
TRANSITION T2
  CONSUME
    P2 : 1;
  PRODUCE
    P3 : 1;
";

pub const LOLA_STRING_NET_WITH_LOOP_TOPOLOGY: &str = "\
PLACE
    P1;

MARKING
TRANSITION T1
  CONSUME
    P1 : 1;
  PRODUCE
    P1 : 1;
";
