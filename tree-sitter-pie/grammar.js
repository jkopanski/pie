// See: https://unicode.org/reports/tr18/#General_Category_Property
const SYMBOL =
  /[\p{Pc}\p{Pd}\p{Pf}\p{Pi}\p{S}]/;
const UPPERCASE =
    /\p{Uppercase_Letter}/;
const LOWERCASE =
    /\p{Lowercase_Letter}/;
const DIGIT =
    /\p{Decimal_Number}/;

const COMMENT =
    /;.*/;

const IDENTIFIER_BODY =
    choice(
	/\p{L}/,
	DIGIT,
	SYMBOL
);

const VARIABLE_HEAD =
    choice(
	LOWERCASE,
	SYMBOL,
	DIGIT
    );
const VARIABLE =
    token(seq(
	VARIABLE_HEAD,
	repeat(IDENTIFIER_BODY)));

const TYPE_HEAD = UPPERCASE;
const TYPE =
    token(seq(
	TYPE_HEAD,
	repeat(IDENTIFIER_BODY)));

const FUNCTION_HEAD =
    choice(
	"\\",
        "λ"
    );

const FUNCTION_TYPE =
    choice(
	"->",
        "→"
    );

module.exports = grammar({
  name: "pie",

  extras: $ => [
    /(\s|\f)/,
    $.comment
  ],

  rules: {
    source: $ => repeat(choice(
	$.claim,
	$.define,
	$._expr
    )),

    claim: $ =>
        seq(
	    "(",
	    "claim",
	    field("identifier", token(VARIABLE)),
	    field("type", $._expr),
	    ")"
	),

    define: $ =>
        seq(
	    "(",
	    "define",
            field("identifier", token(VARIABLE)),
	    field("body", $._expr),
	    ")"
	),

    _expr: $ =>
        choice(
	  $.atom,
	  $.function,
	  $.function_type,
	  $.application,
	  $.type,
	  $.variable,
        ),

      atom: _ => token(
	  seq("'", repeat1(IDENTIFIER_BODY))
      ),

      function: $ =>
	  seq(
	      "(",
	      FUNCTION_HEAD,
	      seq(
		  "(",
		  field("arguments", repeat1($._expr)),
		  ")"
	      ),
	      field("body", $._expr),
	      ")"
	  ),

      function_type: $ =>
	  seq(
	      "(",
	      FUNCTION_TYPE,
	      field("domain", $._expr),
	      field("codomain", repeat1($._expr)),
	      ")",
	  ),

      application: $ =>
	  seq(
	      "(",
	      field("function", $._expr),
	      field("arguments", repeat1($._expr)),
	      ")",
	  ),
	  
      variable: _ => token(VARIABLE),
      type: _ => token(TYPE),
      comment: _ => token(COMMENT),
   }
});
