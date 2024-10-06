use std::{fmt::Display, io};

use crate::token::{OperatorSymbol, Separator};

pub enum Error {
	InvalidShortArgument(String),
	InvalidLongArgument(String),
	NoOptionContinuation,
	CouldNotOpenFile(io::Error),
	CouldNotReadLine,
	FeatureNotYetImplemented(String),
	InvalidTokenStartChar(char),
	InvalidNumericalLiteralBase(char),
	InvalidDigitForBase(char, u8),
	NumericalLiteralTooLarge,
	InvalidKeyword(String),
	InvalidOperator(String),
	TooManyOpenParentheses,
	TooManyCloseParentheses,
	BlankExpression,
	ParenthesisMismatch(Separator, Separator),
	NoOperatorBase,
	BinaryOperatorNotUsedOnExpressions,
	OperatorUsedOnNothing,
	InvalidPrefixOperatorSymbol(OperatorSymbol),
	InvalidInfixOperatorSymbol(OperatorSymbol),
	FunctionParametersWithoutBody,
	UnterminatedCharLiteral,
	EmptyCharLiteral,
	NothingEscaped,
	InvalidEscapeSequence(String),
	MultipleCharsInCharLiteral,
	UnterminatedStringLiteral,
	MetadataItemWithoutChildNode,
	GlobalAugmentedOperator,
	DiscardedGlobalFunctionCall,
	GlobalAssignmentToNonIdentifier,
	GlobalVariableConflict(String),
	ExpectedIdentifier,
	InvalidDependency,
	TooManyFunctionParameters,
	GlobalLValueAssignment,
	LValueFunctionCall,
	LValueFunctionDefinition,
	MultipleEntryPoints,
	TooManyFunctionArguments,
	LinkNotUsedOnFunction,
	InvalidType,
	InvalidTypeWidth,
	UnableToWriteObject,
	CouldNotGetTarget(String),
	InvalidArchitectureBitWidth(u128),
	UnableToEmitObjectFile(String),
	InvalidLValue,
}

impl Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::InvalidShortArgument(arg) => write!(f, "Invalid short argument \"{}\"", arg),
			Error::InvalidLongArgument(arg) => write!(f, "Invalid long argument \"{}\"", arg),
			Error::NoOptionContinuation => write!(f, "No option continuation"),
			Error::CouldNotOpenFile(error) => write!(f, "Could not open file: {error}"),
			Error::CouldNotReadLine => write!(f, "Could not read line"),
			Error::FeatureNotYetImplemented(feature) => write!(f, "{feature} not yet implemented"),
			Error::InvalidTokenStartChar(c) => write!(f, "Invalid token start character '{c}'"),
			Error::InvalidNumericalLiteralBase(c) => write!(f, "Invalid numerical literal base \"0{c}\""),
			Error::InvalidDigitForBase(c, base) => write!(f, "Invalid digit '{c}' for base {base}"),
			Error::NumericalLiteralTooLarge => write!(f, "Numerical literal too large"),
			Error::InvalidKeyword(keyword) => write!(f, "Invalid keyword \"{keyword}\""),
			Error::InvalidOperator(operator) => write!(f, "Invalid operator \"{operator}\""),
			Error::TooManyCloseParentheses => write!(f, "Too many close parentheses"),
			Error::TooManyOpenParentheses => write!(f, "Too many open parentheses"),
			Error::BlankExpression => write!(f, "Blank expression"),
			Error::ParenthesisMismatch(open, close) =>
				write!(f, "Open '{}' mismatched with close '{}'", open.get_symbol(), close.get_symbol()),
			Error::NoOperatorBase => write!(f, "No operator base"),
			Error::BinaryOperatorNotUsedOnExpressions => write!(f, "Binary operator used on non-expressions"),
			Error::InvalidPrefixOperatorSymbol(symbol) => write!(f, "Invalid prefix operator symbol base \"{}\"", symbol.get_symbol()),
			Error::InvalidInfixOperatorSymbol(symbol) => write!(f, "Invalid infix operator symbol base \"{}\"", symbol.get_symbol()),
			Error::OperatorUsedOnNothing => write!(f, "Operator used on nothing"),
			Error::FunctionParametersWithoutBody => write!(f, "Function parameters without body"),
			Error::UnterminatedCharLiteral => write!(f, "Unterminated char literal"),
			Error::EmptyCharLiteral => write!(f, "Empty char literal"),
			Error::NothingEscaped => write!(f, "Nothing escaped"),
			Error::InvalidEscapeSequence(sequence) => write!(f, "Invalid escape sequence \"{sequence}\""),
			Error::MultipleCharsInCharLiteral => write!(f, "Multiple chars in char literal"),
			Error::UnterminatedStringLiteral => write!(f, "Unterminated string literal"),
			Error::MetadataItemWithoutChildNode => write!(f, "Metadata item without child node"),
			Error::GlobalAugmentedOperator => write!(f, "Augmented operator used in global context"),
			Error::DiscardedGlobalFunctionCall => write!(f, "Discarded global function call"),
			Error::GlobalAssignmentToNonIdentifier => write!(f, "Global assignment to non-identifier"),
			Error::GlobalVariableConflict(name) => write!(f, "Re-assignment to global variable {name}"),
			Error::ExpectedIdentifier => write!(f, "Expected an identifier"),
			Error::InvalidDependency => write!(f, "Invalid or cyclic dependency"),
			Error::TooManyFunctionParameters => write!(f, "Too many function parameters"),
			Error::GlobalLValueAssignment => write!(f, "Global l-value assignment"),
			Error::LValueFunctionCall => write!(f, "L-value function call"),
			Error::LValueFunctionDefinition => write!(f, "L-value function definition"),
			Error::MultipleEntryPoints => write!(f, "Multiple entry points"),
			Error::TooManyFunctionArguments => write!(f, "Too many function arguments"),
			Error::LinkNotUsedOnFunction => write!(f, "Link not used on function"),
			Error::InvalidType => write!(f, "Invalid type"),
			Error::InvalidTypeWidth => write!(f, "Invalid type width"),
			Error::UnableToWriteObject => write!(f, "Unable to write object"),
			Error::UnableToEmitObjectFile(error) => write!(f, "Unable to write object: {error}"),
			Error::CouldNotGetTarget(error) => write!(f, "Could not get target: {error}"),
			Error::InvalidArchitectureBitWidth(width) => write!(f, "Unsupported architecture, bit width of {width}, greater than 64"),
			Error::InvalidLValue => write!(f, "Invalid l-value"),
		}
	}
}