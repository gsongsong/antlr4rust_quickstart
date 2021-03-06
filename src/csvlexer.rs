// Generated from CSV.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
use antlr_rust::atn::ATN;
use antlr_rust::char_stream::CharStream;
use antlr_rust::lexer::{BaseLexer, Lexer, LexerRecog};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::lexer_atn_simulator::{LexerATNSimulator, ILexerATNSimulator};
use antlr_rust::PredictionContextCache;
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::token_source::TokenSource;
use antlr_rust::common_token_factory::TokenFactory;
use antlr_rust::token::*;
use antlr_rust::rule_context::BaseRuleContext;
use antlr_rust::parser_rule_context::{ParserRuleContext,LexerContext,cast};
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};

use std::sync::Arc;
use std::cell::RefCell;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};


		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const TEXT:isize=4; 
		pub const STRING:isize=5;
	pub const channelNames: [&'static str;0+2] = [
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	];

	pub const modeNames: [&'static str;1] = [
		"DEFAULT_MODE"
	];

	pub const ruleNames: [&'static str;5] = [
		"T__0", "T__1", "T__2", "TEXT", "STRING"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;4] = [
		None, Some("','"), Some("'\r'"), Some("'\n'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;6]  = [
		None, None, None, None, Some("TEXT"), Some("STRING")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


pub struct CSVLexer {
	base: BaseLexer<CSVLexerActions>,
//	static { RuntimeMetaData.checkVersion("4.8", RuntimeMetaData.VERSION); }
}

impl Deref for CSVLexer{
	type Target = BaseLexer<CSVLexerActions>;

	fn deref(&self) -> &Self::Target {
		&self.base
	}
}

impl DerefMut for CSVLexer{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.base
	}
}


impl CSVLexer {
    fn get_rule_names(&self) -> &'static [&'static str] {
        &ruleNames
    }
    fn get_literal_names(&self) -> &[Option<&str>] {
        &_LITERAL_NAMES
    }

    fn get_symbolic_names(&self) -> &[Option<&str>] {
        &_SYMBOLIC_NAMES
    }

    fn add_error_listener(&mut self, _listener: Box<dyn ErrorListener>) {
        self.base.add_error_listener(_listener);
    }

    fn remove_error_listeners(&mut self) {
        self.base.remove_error_listeners()
    }

    fn get_grammar_file_name(&self) -> &'static str {
        "CSVLexer.g4"
    }

	pub fn new (input: Box<dyn CharStream>) -> Self {
		antlr_rust::recognizer::check_version("0","1");
    	Self {
			base: BaseLexer::new_base_lexer(
				input,
				LexerATNSimulator::new_lexer_atnsimulator(
					_ATN.clone(),
					_decision_to_DFA.clone(),
					_shared_context_cache.clone(),
				),
				Box::new(CSVLexerActions{})
			)
	    }
	}
}

pub struct CSVLexerActions {
}

impl CSVLexerActions{
}

impl LexerRecog for CSVLexerActions{
}

impl Recognizer for CSVLexerActions {}

impl Actions for CSVLexerActions{
	type Recog = BaseLexer<CSVLexerActions>;
	}

	impl CSVLexerActions{

}

impl TokenSource for CSVLexer {
    fn next_token(&mut self) -> Box<dyn Token> {
        self.base.next_token()
    }

    fn get_line(&self) -> isize {
        self.base.get_line()
    }

    fn get_char_position_in_line(&self) -> isize {
        self.base.get_char_position_in_line()
    }

    fn get_input_stream(&mut self) -> &mut dyn CharStream {
        self.base.get_input_stream()
    }

	fn get_source_name(&self) -> String {
		self.base.get_source_name()
	}

    fn get_token_factory(&self) -> &dyn TokenFactory {
        self.base.get_token_factory()
    }
}



	lazy_static! {
	    static ref _ATN: Arc<ATN> =
	        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
	    static ref _decision_to_DFA: Arc<Vec<DFA>> = {
	        let mut dfa = Vec::new();
	        let size = _ATN.decision_to_state.len();
	        for i in 0..size {
	            dfa.push(DFA::new(
	                _ATN.clone(),
	                _ATN.get_decision_state(i),
	                i as isize,
	            ))
	        }
	        Arc::new(dfa)
	    };
	}



	const _serializedATN:&'static str =
		"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x02\
		\x07\x23\x08\x01\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\
		\x09\x05\x04\x06\x09\x06\x03\x02\x03\x02\x03\x03\x03\x03\x03\x04\x03\x04\
		\x03\x05\x06\x05\x15\x0a\x05\x0d\x05\x0e\x05\x16\x03\x06\x03\x06\x03\x06\
		\x03\x06\x07\x06\x1d\x0a\x06\x0c\x06\x0e\x06\x20\x0b\x06\x03\x06\x03\x06\
		\x02\x02\x07\x03\x03\x05\x04\x07\x05\x09\x06\x0b\x07\x03\x02\x04\x06\x02\
		\x0c\x0c\x0f\x0f\x24\x24\x2e\x2e\x03\x02\x24\x24\x02\x25\x02\x03\x03\x02\
		\x02\x02\x02\x05\x03\x02\x02\x02\x02\x07\x03\x02\x02\x02\x02\x09\x03\x02\
		\x02\x02\x02\x0b\x03\x02\x02\x02\x03\x0d\x03\x02\x02\x02\x05\x0f\x03\x02\
		\x02\x02\x07\x11\x03\x02\x02\x02\x09\x14\x03\x02\x02\x02\x0b\x18\x03\x02\
		\x02\x02\x0d\x0e\x07\x2e\x02\x02\x0e\x04\x03\x02\x02\x02\x0f\x10\x07\x0f\
		\x02\x02\x10\x06\x03\x02\x02\x02\x11\x12\x07\x0c\x02\x02\x12\x08\x03\x02\
		\x02\x02\x13\x15\x0a\x02\x02\x02\x14\x13\x03\x02\x02\x02\x15\x16\x03\x02\
		\x02\x02\x16\x14\x03\x02\x02\x02\x16\x17\x03\x02\x02\x02\x17\x0a\x03\x02\
		\x02\x02\x18\x1e\x07\x24\x02\x02\x19\x1a\x07\x24\x02\x02\x1a\x1d\x07\x24\
		\x02\x02\x1b\x1d\x0a\x03\x02\x02\x1c\x19\x03\x02\x02\x02\x1c\x1b\x03\x02\
		\x02\x02\x1d\x20\x03\x02\x02\x02\x1e\x1c\x03\x02\x02\x02\x1e\x1f\x03\x02\
		\x02\x02\x1f\x21\x03\x02\x02\x02\x20\x1e\x03\x02\x02\x02\x21\x22\x07\x24\
		\x02\x02\x22\x0c\x03\x02\x02\x02\x06\x02\x16\x1c\x1e\x02";

