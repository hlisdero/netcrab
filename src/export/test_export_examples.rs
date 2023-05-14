//! # Output examples
//!
//! These strings are used in the unit tests
//! of the corresponding submodules of export.
//!
//! They should not be modified and they showcase
//! how the output looks like in different cases.

pub const DOT_STRING_EMPTY_NET: &str = "digraph petrinet {\n}\n";

pub const DOT_STRING_ONLY_EMPTY_PLACES_NET: &str = r#"digraph petrinet {
    P1 [shape="circle" xlabel="P1" label=""];
    P2 [shape="circle" xlabel="P2" label=""];
    P3 [shape="circle" xlabel="P3" label=""];
    P4 [shape="circle" xlabel="P4" label=""];
    P5 [shape="circle" xlabel="P5" label=""];
}
"#;

pub const DOT_STRING_MARKED_PLACES_NET: &str = r#"digraph petrinet {
    P1 [shape="circle" xlabel="P1" label="•••••"];
    P2 [shape="circle" xlabel="P2" label="6"];
    P3 [shape="circle" xlabel="P3" label="•••"];
    P4 [shape="circle" xlabel="P4" label="••"];
    P5 [shape="circle" xlabel="P5" label="•"];
}
"#;

pub const DOT_STRING_ONLY_EMPTY_TRANSITIONS_NET: &str = r#"digraph petrinet {
    T1 [shape="box" xlabel="" label="T1"];
    T2 [shape="box" xlabel="" label="T2"];
    T3 [shape="box" xlabel="" label="T3"];
    T4 [shape="box" xlabel="" label="T4"];
    T5 [shape="box" xlabel="" label="T5"];
}
"#;

pub const DOT_STRING_NET_WITH_CHAIN_TOPOLOPY: &str = r#"digraph petrinet {
    P1 [shape="circle" xlabel="P1" label=""];
    P2 [shape="circle" xlabel="P2" label=""];
    P3 [shape="circle" xlabel="P3" label=""];
    T1 [shape="box" xlabel="" label="T1"];
    T2 [shape="box" xlabel="" label="T2"];
    P1 -> T1;
    P2 -> T2;
    T1 -> P2;
    T2 -> P3;
}
"#;

pub const DOT_STRING_NET_WITH_LOOP_TOPOLOGY: &str = r#"digraph petrinet {
    P1 [shape="circle" xlabel="P1" label=""];
    T1 [shape="box" xlabel="" label="T1"];
    P1 -> T1;
    T1 -> P1;
}
"#;

pub const LOLA_STRING_ONLY_EMPTY_PLACES_NET: &str = r#"PLACE
    P1,
    P2,
    P3,
    P4,
    P5;

MARKING
    P1 : 0,
    P2 : 0,
    P3 : 0,
    P4 : 0,
    P5 : 0;

"#;

pub const LOLA_STRING_MARKED_PLACES_NET: &str = r#"PLACE
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

"#;

pub const LOLA_STRING_ONLY_EMPTY_TRANSITIONS_NET: &str = r#"TRANSITION T1
  CONSUME;
  PRODUCE;
TRANSITION T2
  CONSUME;
  PRODUCE;
TRANSITION T3
  CONSUME;
  PRODUCE;
TRANSITION T4
  CONSUME;
  PRODUCE;
TRANSITION T5
  CONSUME;
  PRODUCE;
"#;

pub const LOLA_STRING_NET_WITH_CHAIN_TOPOLOPY: &str = r#"PLACE
    P1,
    P2,
    P3;

MARKING
    P1 : 0,
    P2 : 0,
    P3 : 0;

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
"#;

pub const LOLA_STRING_NET_WITH_LOOP_TOPOLOGY: &str = r#"PLACE
    P1;

MARKING
    P1 : 0;

TRANSITION T1
  CONSUME
    P1 : 1;
  PRODUCE
    P1 : 1;
"#;

pub const PNML_STRING_EMPTY_NET: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0" />
  </net>
</pnml>"#;

pub const PNML_STRING_ONLY_EMPTY_PLACES_NET: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="P1">
        <name>
          <text>P1</text>
        </name>
      </place>
      <place id="P2">
        <name>
          <text>P2</text>
        </name>
      </place>
      <place id="P3">
        <name>
          <text>P3</text>
        </name>
      </place>
      <place id="P4">
        <name>
          <text>P4</text>
        </name>
      </place>
      <place id="P5">
        <name>
          <text>P5</text>
        </name>
      </place>
    </page>
  </net>
</pnml>"#;

pub const PNML_STRING_MARKED_PLACES_NET: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="P1">
        <name>
          <text>P1</text>
        </name>
        <initialMarking>
          <text>5</text>
        </initialMarking>
      </place>
      <place id="P2">
        <name>
          <text>P2</text>
        </name>
        <initialMarking>
          <text>6</text>
        </initialMarking>
      </place>
      <place id="P3">
        <name>
          <text>P3</text>
        </name>
        <initialMarking>
          <text>3</text>
        </initialMarking>
      </place>
      <place id="P4">
        <name>
          <text>P4</text>
        </name>
        <initialMarking>
          <text>2</text>
        </initialMarking>
      </place>
      <place id="P5">
        <name>
          <text>P5</text>
        </name>
        <initialMarking>
          <text>1</text>
        </initialMarking>
      </place>
    </page>
  </net>
</pnml>"#;

pub const PNML_STRING_ONLY_EMPTY_TRANSITIONS_NET: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <transition id="T1">
        <name>
          <text>T1</text>
        </name>
      </transition>
      <transition id="T2">
        <name>
          <text>T2</text>
        </name>
      </transition>
      <transition id="T3">
        <name>
          <text>T3</text>
        </name>
      </transition>
      <transition id="T4">
        <name>
          <text>T4</text>
        </name>
      </transition>
      <transition id="T5">
        <name>
          <text>T5</text>
        </name>
      </transition>
    </page>
  </net>
</pnml>"#;

pub const PNML_STRING_NET_WITH_CHAIN_TOPOLOPY: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="P1">
        <name>
          <text>P1</text>
        </name>
      </place>
      <place id="P2">
        <name>
          <text>P2</text>
        </name>
      </place>
      <place id="P3">
        <name>
          <text>P3</text>
        </name>
      </place>
      <transition id="T1">
        <name>
          <text>T1</text>
        </name>
      </transition>
      <transition id="T2">
        <name>
          <text>T2</text>
        </name>
      </transition>
      <arc source="P1" target="T1" id="(P1, T1)">
        <name>
          <text>(P1, T1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="P2" target="T2" id="(P2, T2)">
        <name>
          <text>(P2, T2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="T1" target="P2" id="(T1, P2)">
        <name>
          <text>(T1, P2)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="T2" target="P3" id="(T2, P3)">
        <name>
          <text>(T2, P3)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;

pub const PNML_STRING_NET_WITH_LOOP_TOPOLOGY: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<pnml xmlns="http://www.pnml.org/version-2009/grammar/pnml">
  <net id="net0" type="http://www.pnml.org/version-2009/grammar/ptnet">
    <page id="page0">
      <place id="P1">
        <name>
          <text>P1</text>
        </name>
      </place>
      <transition id="T1">
        <name>
          <text>T1</text>
        </name>
      </transition>
      <arc source="P1" target="T1" id="(P1, T1)">
        <name>
          <text>(P1, T1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
      <arc source="T1" target="P1" id="(T1, P1)">
        <name>
          <text>(T1, P1)</text>
        </name>
        <inscription>
          <text>1</text>
        </inscription>
      </arc>
    </page>
  </net>
</pnml>"#;
