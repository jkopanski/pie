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

const LAMBDA_HEAD =
    choice(
	"\\",
        "λ",
	token("lambda"),
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
	$.expression
    )),

    claim: $ =>
        seq(
	    "(",
	    token("claim"),
	    field("identifier", $.identifier),
	    field("type", $.expression),
	    ")"
	),

    define: $ =>
        seq(
	    "(",
	    token("define"),
            field("identifier", $.identifier),
	    field("body", $.expression),
	    ")"
	),

    expression: $ =>
        choice(
	  $.atom,
	  $.lambda,
	  // $.function_type,
	  $.application,
	  $.type_identifier,
	  $.identifier,
        ),

      atom: $ => seq(
	  "'",
	  field("identifier", $.identifier)
      ),

      lambda: $ =>
	  seq(
	      "(",
	      LAMBDA_HEAD,
	      seq(
		  "(",
		  field("arguments", repeat($.expression)),
		  ")"
	      ),
	      field("body", $.expression),
	      ")"
	  ),

      // function_type: $ =>
      // 	  seq(
      // 	      "(",
      // 	      FUNCTION_TYPE,
      // 	      field("domain", $.expression),
      // 	      field("codomain", repeat1($.expression)),
      // 	      ")",
      // 	  ),

      application: $ =>
	  seq(
	      "(",
	      field("function", $.expression),
	      field("arguments", repeat($.expression)),
	      ")",
	  ),
	  
      identifier: _ => token(VARIABLE),
      type_identifier: _ => token(TYPE),
      comment: _ => token(COMMENT),
   }
});
