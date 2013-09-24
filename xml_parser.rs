use xml_node::*;
use std::io::*;

mod xml_node;

enum State {
    OutsideTag,
    TagOpened,
    InProcessingInstructions,
    InTagName,
    InCloseTagName,
    InTag,
    InAttrName,
    InAttrValue,
    ExpectDelimiter,
    ExpectClose,
    ExpectSpaceOrClose,
    InExclamationMark,
    InCDATAOpening,
    InCDATA,
    InCommentOpening,
    InComment1,
    InComment2,
    InDoctype,
    Namespace
}

#[deriving(Eq)]
pub enum Character {
    Chars(char),
    NewLine
}

pub struct XmlParser {
    line: uint,
    col: uint,
    depth: uint,
    elem: Option<XmlElem>,
    priv pushback: Option<char>,
    priv source: @Reader,
    priv buf: ~str,
    priv name: ~str,
    priv attrName: ~str,
    priv attributes: ~[XmlAttr],
    priv delim: char,
    priv state: State

}

impl XmlParser {
    /// Constructs a new XmlParser from Reader `data`
    /// The Xmlparser will use the given string as the source for parsing.
    /// Best used for small examples.
    /// ~~~
    /// let mut p = XmlParser::from_read(stdin)
    /// p.parse_doc() => XmlDoc { root: XmlElem {name: "root"} ... }
    /// ~~~
    pub fn from_reader(data : @Reader)
                     -> XmlParser {
        XmlParser {
            line: 1,
            col: 0,
            buf: ~"",
            name: ~"",
            elem: None,
            pushback: None,
            source: data,
            attrName: ~"",
            attributes: ~[],
            delim: 0 as char,
            state: OutsideTag,
            depth: 0
        }
    }

    /// This method will parse entire document into memory as a tree of 
    /// XmlElem. It retuns an XmlDoc if it parses correctly or an Error
    /// if the parsing wasn't succesful.
    // TODO IMPLEMENT
    pub fn parse_doc(&mut self)
                     -> Result<XmlDoc,Error> {
        Ok(XmlDoc::new())
    }
    /// This method pulls tokens in similar way `parse_doc`  does, but 
    /// it also takes an callback to function to execute on each iteration.
    pub fn parse_call(&mut self, cb: &fn (Result<Events,Error>))
                      -> Result<XmlDoc,Error>{
        //TODO IMPLEMENT
        Ok(XmlDoc::new())
    }
    /// This method pulls tokens until it reaches a fully formed XML node
    /// once it's finished a node, it stops returning said node or error
    /// if it encountered one.
    ///
    /// This method should be used similar to an outer iterator.
    pub fn next(&mut self)
                -> Result<XmlNode,Error>{
        //TODO IMPLEMENT
        let retVal = Ok(XmlCDATA(~"CDATA"));

        retVal

    }

    /// This method reads a character and returns an enum that might be
    /// either a value of character, a new-line sign or a restricted
    /// character.
    fn read(&mut self)
            -> Character {
        //TODO implement docs and restricted chars
        let chr = self.raw_read();
        let retVal;
        match chr {
            '\r' => {
                self.line += 1u;
                self.col = 0u;

                let chrPeek = self.raw_read();
                if(chrPeek != '\x85' && chrPeek != '\n'){
                    self.raw_unread();
                }

                retVal = NewLine;

            },
            '\x85'
            | '\u2028' => {
                self.line += 1u;
                self.col = 0u;
                retVal = NewLine;
            },
            _ => {
                self.col += 1u;
                retVal = Chars(chr);
            }


        }
        retVal
    }

    /// This method verifies if the character is a restricted character
    /// According to http://www.w3.org/TR/xml11/#NT-Char
    /// Restricted character include anything in the range of
    /// [#x1-#x8], [#xB-#xC], [#xE-#x1F], [#x7F-#x84], [#x86-#x9F]
    /// [#x1FFFE-#x1FFFF], [#x2FFFE-#x2FFFF], [#x3FFFE-#x3FFFF],
    /// [#x4FFFE-#x4FFFF], [#x5FFFE-#x5FFFF], [#x6FFFE-#x6FFFF],
    /// [#x7FFFE-#x7FFFF], [#x8FFFE-#x8FFFF], [#x9FFFE-#x9FFFF],
    /// [#xAFFFE-#xAFFFF], [#xBFFFE-#xBFFFF], [#xCFFFE-#xCFFFF],
    /// [#xDFFFE-#xDFFFF], [#xEFFFE-#xEFFFF], [#xFFFFE-#xFFFFF],
    /// [#x10FFFE-#x10FFFF].
    fn is_restricted(c: char) -> bool {
        match c {
            '\x01'..'\x08'
            | '\x0B'.. '\x0C'
            | '\x0E'.. '\x1F'
            | '\x7F'.. '\x84'
            | '\x86'.. '\x9F'
            | '\U0001FFFE' | '\U0001FFFF'
            | '\U0002FFFE' | '\U0002FFFF'
            | '\U0003FFFE' | '\U0003FFFF'
            | '\U0004FFFE' | '\U0004FFFF'
            | '\U0005FFFE' | '\U0005FFFF'
            | '\U0006FFFE' | '\U0006FFFF'
            | '\U0007FFFE' | '\U0007FFFF'
            | '\U0008FFFE' | '\U0008FFFF'
            | '\U0009FFFE' | '\U0009FFFF'
            | '\U000AFFFE' | '\U000AFFFF'
            | '\U000BFFFE' | '\U000BFFFF'
            | '\U000CFFFE' | '\U000CFFFF'
            | '\U000DFFFE' | '\U000DFFFF'
            | '\U000EFFFE' | '\U000EFFFF'
            | '\U000FFFFE' | '\U000FFFFF' => true,
            _ => false
        }
    }

    #[inline]
    /// This method reads the source andBb simply updates position
    /// This method WILL NOT update new col or row
    fn raw_read(&mut self) -> char {
        self.source.read_char()
    }

    #[inline]
    /// This method unreads the source and simply updates position
    /// This method WILL NOT update new col or row
    fn raw_unread(&mut self) {
        self.source.seek(-1, SeekCur);
    }
}


pub fn main() {
    error!("This is an error log");
    warn!("This is a warn log");
    info!("this is an info log");
    debug!("This is a debug log");
}


#[cfg(test)]
mod tests{
    use super::*;
    use std::io::*;

    #[test]
    fn test_is_restricted(){
        assert_eq!(true, XmlParser::is_restricted('\x0B'));
        assert_eq!(true, XmlParser::is_restricted('\x02'));
        assert_eq!(true, XmlParser::is_restricted('\x0C'));
        assert_eq!(true, XmlParser::is_restricted('\x0F'));
        assert_eq!(true, XmlParser::is_restricted('\x1F'));
        assert_eq!(true, XmlParser::is_restricted('\x7F'));
        assert_eq!(true, XmlParser::is_restricted('\x84'));
        assert_eq!(true, XmlParser::is_restricted('\x86'));
        assert_eq!(true, XmlParser::is_restricted('\x9A'));
        assert_eq!(true, XmlParser::is_restricted('\U0001FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0001FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0002FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0002FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0003FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0003FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0004FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0004FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0005FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0005FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0006FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0006FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0007FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0007FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0008FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0008FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U0009FFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U0009FFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U000AFFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U000AFFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U000BFFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U000BFFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U000CFFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U000CFFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U000DFFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U000DFFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U000EFFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U000EFFFF'));
        assert_eq!(true, XmlParser::is_restricted('\U000FFFFE'));
        assert_eq!(true, XmlParser::is_restricted('\U000FFFFF'));

    }

    #[test]
    fn test_read_newline(){
        let r1 = @BytesReader {
                bytes : "a\r\nt".as_bytes(),
                pos: @mut 0
        } as @Reader;

        let mut parser = XmlParser::from_reader(r1);
        assert_eq!(Chars('a'), parser.read());
        assert_eq!(1,   parser.line);
        assert_eq!(1,   parser.col);
        assert_eq!(NewLine,parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(0,   parser.col);
        assert_eq!(Chars('t'),parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(1,   parser.col);

        let r2= @BytesReader {
                bytes : "a\rt".as_bytes(),
                pos: @mut 0
        } as @Reader;

        parser = XmlParser::from_reader(r2);
        assert_eq!(Chars('a'), parser.read());
        assert_eq!(1,   parser.line);
        assert_eq!(1,   parser.col);
        assert_eq!(NewLine,parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(0,   parser.col);
        assert_eq!(Chars('t'),parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(1,   parser.col);

        let r3 = @BytesReader {
                bytes : "a\r\x85t".as_bytes(),
                pos: @mut 0
        } as @Reader;

        parser = XmlParser::from_reader(r3);
        assert_eq!(Chars('a'), parser.read());
        assert_eq!(1,   parser.line);
        assert_eq!(1,   parser.col);
        assert_eq!(NewLine, parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(0,   parser.col);
        assert_eq!(Chars('t'),parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(1,   parser.col);


        let r4 = @BytesReader {
                bytes : "a\x85t".as_bytes(),
                pos: @mut 0
        } as @Reader;

        let mut parser = XmlParser::from_reader(r4);
        assert_eq!(Chars('a'), parser.read());
        assert_eq!(1,   parser.line);
        assert_eq!(1,   parser.col);
        assert_eq!(NewLine,parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(0,   parser.col);
        assert_eq!(Chars('t'),parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(1,   parser.col);
      

        let r5 = @BytesReader {
                bytes : "a\u2028t".as_bytes(),
                pos: @mut 0
        } as @Reader;

        let mut parser = XmlParser::from_reader(r5);
        assert_eq!(Chars('a'), parser.read());
        assert_eq!(1,   parser.line);
        assert_eq!(1,   parser.col);
        assert_eq!(NewLine,parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(0,   parser.col);
        assert_eq!(Chars('t'),parser.read());
        assert_eq!(2,   parser.line);
        assert_eq!(1,   parser.col);
    }

}
