# sql

## Overview

This page provides the ANTLR4 grammar specification for CoinbaSeQL (pronounced “coinbase QL”), the SQL dialect used by the SQL API.

CoinbaSeQL supports all standard SQL query features. For practical examples and usage, see:

-   **[Quickstart](https://developer.chrome.com/data/sql-api/quickstart)** - Try queries in the SQL Playground
-   **[Schema Reference](https://developer.chrome.com/data/sql-api/schema)** - Explore available tables and columns
-   **[FAQ](https://developer.chrome.com/data/sql-api/faq)** - Common questions about supported features

The complete ANTLR4 grammar specification below defines exactly what SQL syntax is supported by CoinbaSeQL.

### Design Principles

CoinbaSeQL is created with the following principles:

-   As similar to standard SQL as possible
-   Support all common SQL features per the SQL standard
-   Provide understandable, actionable, and helpful error messages

### Grammar Specification

You can also retrieve this grammar programmatically via the [Get SQL Grammar endpoint](https://developer.chrome.com/api-reference/v2/rest-api/sql-api/get-sql-grammar).

```
grammar SqlQuery;
// If you update this grammar, simply run `make gen` from the top-level to update the parsing logic.
// Inspired by ClickHouse parser and lexer:
// https://github.com/abyss7/ClickHouse/blob/master/src/Parsers/New/ClickHouseParser.g4
// https://github.com/abyss7/ClickHouse/blob/master/src/Parsers/New/ClickHouseLexer.g4
// Parser rules
query: cteClause? unionStatement SEMICOLON? EOF;
unionStatement:
	unionSelect (unionOperator unionSelect)* (
		ORDER BY orderByElements
	)? (LIMIT limitClause)?;
unionSelect: selectStatement | LPAREN selectStatement RPAREN;
unionOperator: UNION ALL | UNION DISTINCT | UNION;
cteClause: WITH cteDefinition (COMMA cteDefinition)*;
cteDefinition:
	cteName (LPAREN columnList RPAREN)? AS LPAREN selectStatement RPAREN;
cteName: identifier;
columnList: identifier (COMMA identifier)*;
selectStatement:
	SELECT (DISTINCT)? selectElements FROM tableExpression (
		WHERE condition
	)? (GROUP BY groupByElements)? (ORDER BY orderByElements)? (
		LIMIT limitClause
	)?
	| SELECT (DISTINCT)? selectElements // For literals/expressions without FROM
	(ORDER BY orderByElements)? (LIMIT limitClause)?;
selectElements: STAR | selectElement (COMMA selectElement)*;
selectElement: expression (AS? alias)? | tableWildcard;
tableWildcard: (identifier DOT)? STAR;
tableExpression: tableReference (joinExpression)*;
tableReference:
	tableOrCteReference (AS? alias)?
	| LPAREN selectStatement RPAREN (AS? alias)?
	| LPAREN unionStatement RPAREN (AS? alias)?;
tableOrCteReference: tableName | identifier;
joinExpression: joinType? JOIN tableReference ON condition;
joinType: INNER | LEFT | RIGHT | FULL;
condition: expression;
groupByElements: expression (COMMA expression)*;
orderByElements: orderByElement (COMMA orderByElement)*;
orderByElement: expression (ASC | DESC)?;
limitClause: INTEGER_LITERAL;
expression:
	expression BETWEEN expression AND expression
	| expression IN LPAREN (expressionList | selectStatement) RPAREN
	| expression IS (NOT)? NULL
	| expression binaryOperator expression
	| expression CAST_OP dataType // PostgreSQL-style casting (e.g., 1::Int32)
	| expression DOT identifier // Dot notation
	| expression LBRACKET expression RBRACKET // Array/map indexing
	| functionCall
	| castExpression // Standard SQL CAST function
	| LPAREN expression RPAREN
	| CASE (expression)? whenClause+ (ELSE expression)? END
	| primaryExpression;
castExpression: CAST LPAREN expression AS dataType RPAREN;
dataType:
	identifier (LPAREN typeArguments RPAREN)?
	| ARRAY LPAREN dataType RPAREN // Array(Int32)
	| MAP LPAREN dataType COMMA dataType RPAREN // Map(String, String)
	| TUPLE LPAREN dataType (COMMA dataType)* RPAREN; // Tuple(Int32, String)
typeArguments: typeArgument (COMMA typeArgument)*;
typeArgument: dataType | INTEGER_LITERAL;
whenClause: WHEN expression THEN expression;
expressionList: expression (COMMA expression)*;
primaryExpression:
	columnReference
	| literal
	| arrayLiteral // Array literal [1, 2, 3]
	| mapLiteral // Map literal {'key': 'value'}
	| tupleLiteral // Tuple literal (1, 'a', true)
	| LPAREN selectStatement RPAREN; // Subquery as primary expression
columnReference: (tableOrCtePrefix DOT)? columnName;
tableOrCtePrefix: tableName | identifier;
functionCall: identifier LPAREN functionArgs? RPAREN;
lambda: lambdaParams ARROW expression;
lambdaParams:
	identifier
	| LPAREN (identifier (COMMA identifier)*)? RPAREN;
functionArgs:
	STAR
	| DISTINCT expressionList
	| lambda (COMMA expressionList)?
	| expressionList;
binaryOperator:
	EQ
	| NEQ
	| LT
	| LE
	| GT
	| GE
	| PLUS
	| MINUS
	| STAR
	| DIV
	| MOD
	| AND
	| OR
	| LIKE;
literal:
	STRING_LITERAL
	| INTEGER_LITERAL
	| DECIMAL_LITERAL
	| NULL
	| TRUE
	| FALSE;
arrayLiteral:
	LBRACKET (expression (COMMA expression)*)? RBRACKET;
mapLiteral:
	LBRACE (mapEntry (COMMA mapEntry)*)? RBRACE
	| MAP LPAREN (mapPair (COMMA mapPair)*)? RPAREN;
mapEntry: expression COLON expression;
mapPair: expression COMMA expression;
tupleLiteral:
	LPAREN expression (COMMA expression)+ RPAREN // Requires at least 2 elements
	| TUPLE LPAREN (expression (COMMA expression)*)? RPAREN;
tableName: identifier (DOT identifier)?;
columnName: identifier;
functionName: identifier;
alias: identifier;
identifier: IDENTIFIER | QUOTED_IDENTIFIER | keyword;
// All keywords that can potentially be used as identifiers
keyword:
	SELECT
	| FROM
	| WHERE
	| GROUP
	| BY
	| ORDER
	| LIMIT
	| AS
	| JOIN
	| ON
	| INNER
	| LEFT
	| RIGHT
	| FULL
	| AND
	| OR
	| NOT
	| IN
	| BETWEEN
	| LIKE
	| IS
	| NULL
	| TRUE
	| FALSE
	| CASE
	| WHEN
	| THEN
	| ELSE
	| END
	| DISTINCT
	| ASC
	| DESC
	| CAST
	| WITH
	| UNION
	| ALL
	| ARRAY
	| MAP
	| TUPLE
	| OFFSET
	| OUTER;
// Lexer rules - Keywords
SELECT: S E L E C T;
FROM: F R O M;
WHERE: W H E R E;
GROUP: G R O U P;
BY: B Y;
ORDER: O R D E R;
LIMIT: L I M I T;
AS: A S;
JOIN: J O I N;
ON: O N;
INNER: I N N E R;
LEFT: L E F T;
RIGHT: R I G H T;
FULL: F U L L;
AND: A N D;
OR: O R;
NOT: N O T;
IN: I N;
BETWEEN: B E T W E E N;
LIKE: L I K E;
IS: I S;
NULL: N U L L;
TRUE: T R U E;
FALSE: F A L S E;
CASE: C A S E;
WHEN: W H E N;
THEN: T H E N;
ELSE: E L S E;
END: E N D;
DISTINCT: D I S T I N C T;
ASC: A S C;
DESC: D E S C;
CAST: C A S T;
WITH: W I T H;
UNION: U N I O N;
ALL: A L L;
ARRAY: A R R A Y;
MAP: M A P;
TUPLE: T U P L E;
OFFSET: O F F S E T;
OUTER: O U T E R;
// Lexer rules - Comparison Operators
EQ: '=';
NEQ: '!=' | '<>';
LT: '<';
GT: '>';
LE: '<=';
GE: '>=';
// Lexer rules - Arithmetic Operators
PLUS: '+';
MINUS: '-';
STAR: '*';
DIV: '/';
MOD: '%';
ARROW: '->';
// Lexer rules - Delimiters
LPAREN: '(';
RPAREN: ')';
COMMA: ',';
SEMICOLON: ';';
DOT: '.';
LBRACKET: '[';
RBRACKET: ']';
LBRACE: '{';
RBRACE: '}';
COLON: ':';
CAST_OP: '::';
// Lexer rules - Literals
STRING_LITERAL: '\'' (~['])* '\'';
INTEGER_LITERAL: [0-9]+;
DECIMAL_LITERAL: [0-9]+ '.' [0-9]* | '.' [0-9]+;
IDENTIFIER: [a-zA-Z_] [a-zA-Z_0-9]*;
QUOTED_IDENTIFIER:
	'"' (~'"' | '""')* '"'
	| '`' (~'`' | '``')* '`';
// Whitespace and comments
WS: [ \t\r\n]+ -> skip;
COMMENT: '--' ~[\r\n]* -> skip;
MULTI_LINE_COMMENT: '/*' .*? '*/' -> skip;
// Case-insensitive matching fragments
fragment A: [aA];
fragment B: [bB];
fragment C: [cC];
fragment D: [dD];
fragment E: [eE];
fragment F: [fF];
fragment G: [gG];
fragment H: [hH];
fragment I: [iI];
fragment J: [jJ];
fragment K: [kK];
fragment L: [lL];
fragment M: [mM];
fragment N: [nN];
fragment O: [oO];
fragment P: [pP];
fragment Q: [qQ];
fragment R: [rR];
fragment S: [sS];
fragment T: [tT];
fragment U: [uU];
fragment V: [vV];
fragment W: [wW];
fragment X: [xX];
fragment Y: [yY];
fragment Z: [zZ];

```

When using AI assistants to write SQL queries:

1.  **Provide context**: Give your AI tool both this grammar specification and the [schema reference](https://developer.chrome.com/data/sql-api/schema)
2.  **Be specific**: Ask for queries that match your specific use case (e.g., “Write a query to find all USDC transfers over $1000 in the last 24 hours”)
3.  **Validate**: Always test AI-generated queries in the [SQL Playground](https://portal.cdp.coinbase.com/products/data/playground) before using them in production