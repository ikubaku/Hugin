use std::error::Error;
use std::path::Path;

use log::{debug, error, info, warn};

use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1, line_ending, multispace0, not_line_ending};
use nom::combinator::{eof, map, map_res, not, peek};
use nom::error::{FromExternalError, ParseError};
use nom::lib::std::collections::HashMap;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

use crate::clone_pair::{ClonePair, CodePosition, CodeSlice};
use crate::error::{FileNotFoundFromResultError, InvalidCCFinderSWResult};

enum DataBlock {
    FileDescription(HashMap<(u32, u32), String>),
    Clone(Vec<CloneSet>),
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
struct SetElement {
    file_number: (u32, u32),
    start_position: CodePosition,
    end_position: CodePosition,
    lnr: u32,
}

impl SetElement {
    pub fn new(
        file_number: (u32, u32),
        start_position: CodePosition,
        end_position: CodePosition,
        lnr: u32,
    ) -> Self {
        SetElement {
            file_number,
            start_position,
            end_position,
            lnr,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct CloneSet {
    elements: Vec<SetElement>,
}

impl CloneSet {
    pub fn new(elements: Vec<SetElement>) -> Self {
        CloneSet { elements }
    }
}

#[derive(Debug, PartialEq)]
pub struct ParsedResult {
    file_description: HashMap<(u32, u32), String>,
    clone: Vec<CloneSet>,
}

impl ParsedResult {
    fn get_file_number_from_file_name(
        &self,
        file_name: &str,
    ) -> Result<(u32, u32), FileNotFoundFromResultError> {
        for (k, v) in self.file_description.iter() {
            let path = Path::new(v.as_str());
            if path.file_name().unwrap().to_str().unwrap() == file_name {
                return Ok(k.clone());
            }
        }
        Err(FileNotFoundFromResultError)
    }
    fn new(file_description: HashMap<(u32, u32), String>, clone: Vec<CloneSet>) -> Self {
        ParsedResult {
            file_description,
            clone,
        }
    }

    pub fn get_clone_pairs(
        &self,
        project_file_name: &str,
        example_source_name: &str,
    ) -> Result<Vec<ClonePair>, Box<dyn Error>> {
        let project_file_number = self.get_file_number_from_file_name(project_file_name)?;
        debug!("project_file_number: {:?}", project_file_number);
        let example_source_file_number =
            self.get_file_number_from_file_name(example_source_name)?;
        debug!(
            "example_source_file_number: {:?}",
            example_source_file_number
        );
        let mut res = Vec::new();
        for s in &self.clone {
            let mut project_code_part = None;
            let mut example_code_part = None;
            for e in &s.elements {
                debug!("SetElement: {:?}", e);
                project_code_part = if e.file_number == project_file_number {
                    debug!("Reached project_code_part set.");
                    if project_code_part.is_none() {
                        Ok(Some(CodeSlice::new(
                            e.start_position.clone(),
                            e.end_position.clone(),
                        )))
                    } else {
                        Err(InvalidCCFinderSWResult::new(
                            "Duplicated project code part entries.",
                        ))
                    }
                } else {
                    Ok(project_code_part)
                }?;
                example_code_part = if e.file_number == example_source_file_number {
                    debug!("Reached example_code_part set.");
                    if example_code_part.is_none() {
                        Ok(Some(CodeSlice::new(
                            e.start_position.clone(),
                            e.end_position.clone(),
                        )))
                    } else {
                        Err(InvalidCCFinderSWResult::new(
                            "Duplicated example code part entries.",
                        ))
                    }
                } else {
                    Ok(example_code_part)
                }?;
            }
            debug!("project_code_part: {:?}", project_code_part);
            debug!("example_code_part: {:?}", example_code_part);
            let new_pair = if project_code_part.is_some() && example_code_part.is_some() {
                Ok(ClonePair::new(
                    project_code_part.unwrap(),
                    example_code_part.unwrap(),
                ))
            } else {
                Err(InvalidCCFinderSWResult::new(
                    "Missing mandatory code part entries.",
                ))
            }?;
            res.push(new_pair);
        }
        Ok(res)
    }
}

pub struct ResultParser {}

impl ResultParser {
    fn parse_digits<'a, E>(&self, input: &'a str) -> IResult<&'a str, u32, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        map_res(
            preceded(|i| self.parse_preceding_whitespace(i), digit1),
            move |val| u32::from_str_radix(val, 10),
        )(input)
    }

    fn parse_string<'a, E>(&self, input: &'a str) -> IResult<&'a str, String, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let string_parser = delimited(
            |i| self.parse_preceding_whitespace(i),
            not_line_ending,
            line_ending,
        );
        map_res(string_parser, move |val| Ok(String::from(val)))(input)
    }

    fn parse_preceding_whitespace<'a, E>(&self, input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let preceding_whitespace_parser =
            tuple((peek(not(preceded(multispace0, eof))), multispace0));
        map_res(preceding_whitespace_parser, |_| Ok(()))(input)
    }

    fn parse_columns<'a, E>(&self, input: &'a str) -> IResult<&'a str, u32, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        self.parse_digits(input)
    }

    fn parse_lnr<'a, E>(&self, input: &'a str) -> IResult<&'a str, u32, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        self.parse_digits(input)
    }

    fn parse_tokens<'a, E>(&self, input: &'a str) -> IResult<&'a str, u32, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        self.parse_digits(input)
    }

    fn parse_lines<'a, E>(&self, input: &'a str) -> IResult<&'a str, u32, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        self.parse_digits(input)
    }

    fn parse_filename<'a, E>(&self, input: &'a str) -> IResult<&'a str, String, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        self.parse_string(input)
    }

    fn parse_position<'a, E>(&self, input: &'a str) -> IResult<&'a str, CodePosition, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let position_parser = tuple((
            |i| self.parse_digits(i),
            preceded(char(','), |i| self.parse_digits(i)),
            preceded(char(','), |i| self.parse_digits(i)),
        ));
        map_res(position_parser, move |val: (u32, u32, u32)| {
            Ok(CodePosition::new(val.0, val.1))
        })(input)
    }

    fn parse_file_number<'a, E>(&self, input: &'a str) -> IResult<&'a str, (u32, u32), E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let position_parser = tuple((
            |i| self.parse_digits(i),
            preceded(char('.'), |i| self.parse_digits(i)),
        ));
        map_res(position_parser, move |val: (u32, u32)| Ok(val))(input)
    }

    fn parse_set_element<'a, E>(&self, input: &'a str) -> IResult<&'a str, SetElement, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let set_element_parser = preceded(
            |i| self.parse_preceding_whitespace(i),
            tuple((
                |i| self.parse_file_number(i),
                |i| self.parse_position(i),
                |i| self.parse_position(i),
                |i| self.parse_lnr(i),
            )),
        );
        map_res(
            set_element_parser,
            move |val: ((u32, u32), CodePosition, CodePosition, u32)| {
                Ok(SetElement::new(val.0, val.1, val.2, val.3))
            },
        )(input)
    }

    fn parse_set<'a, E>(&self, input: &'a str) -> IResult<&'a str, Vec<SetElement>, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let set_parser = delimited(
            preceded(|i| self.parse_preceding_whitespace(i), tag("#begin{set}")),
            many1(|i| self.parse_set_element(i)),
            preceded(|i| self.parse_preceding_whitespace(i), tag("#end{set}")),
        );
        map_res(set_parser, move |val: Vec<SetElement>| Ok(val))(input)
    }

    fn parse_clone<'a, E>(&self, input: &'a str) -> IResult<&'a str, Vec<CloneSet>, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let clone_parser = delimited(
            preceded(|i| self.parse_preceding_whitespace(i), tag("#begin{clone}")),
            many1(|i| self.parse_set(i)),
            preceded(|i| self.parse_preceding_whitespace(i), tag("#end{clone}")),
        );
        map_res(clone_parser, move |val: Vec<Vec<SetElement>>| {
            let mut res = Vec::new();
            for e in val {
                res.push(CloneSet::new(e));
            }
            Ok(res)
        })(input)
    }

    fn parse_file_description_entry<'a, E>(
        &self,
        input: &'a str,
    ) -> IResult<&'a str, ((u32, u32), String), E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let file_description_entry_parser = preceded(
            |i| self.parse_preceding_whitespace(i),
            tuple((
                |i| self.parse_file_number(i),
                |i| self.parse_lines(i),
                |i| self.parse_tokens(i),
                |i| self.parse_filename(i),
            )),
        );
        map_res(
            file_description_entry_parser,
            move |val: ((u32, u32), u32, u32, String)| Ok((val.0, val.3)),
        )(input)
    }

    fn parse_file_description<'a, E>(
        &self,
        input: &'a str,
    ) -> IResult<&'a str, HashMap<(u32, u32), String>, E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let file_description_parser = delimited(
            preceded(
                |i| self.parse_preceding_whitespace(i),
                tag("#begin{file description}"),
            ),
            many1(|i| self.parse_file_description_entry(i)),
            preceded(
                |i| self.parse_preceding_whitespace(i),
                tag("#end{file description}"),
            ),
        );

        map_res(
            file_description_parser,
            move |val: Vec<((u32, u32), String)>| {
                let mut res = HashMap::<(u32, u32), String>::new();
                for e in val {
                    res.insert(e.0, e.1);
                }
                Ok(res)
            },
        )(input)
    }

    fn parse_block<'a, E>(&self, input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let block_parser = delimited(
            preceded(|i| self.parse_preceding_whitespace(i), tag("#begin{")),
            take_until("#end{"),
            not_line_ending,
        );
        map_res(block_parser, move |_| Ok(()))(input)
    }

    fn parse_tag<'a, E>(&self, input: &'a str) -> IResult<&'a str, (), E>
    where
        E: ParseError<&'a str> + FromExternalError<&'a str, std::num::ParseIntError>,
    {
        let tag_parser = preceded(
            preceded(|i| self.parse_preceding_whitespace(i), char('#')),
            not_line_ending,
        );
        map_res(tag_parser, move |_| Ok(()))(input)
    }

    pub fn parse_result<'a, E>(&self, input: &'a str) -> IResult<&'a str, ParsedResult, E>
    where
        E: ParseError<&'a str>
            + FromExternalError<&'a str, std::num::ParseIntError>
            + FromExternalError<&'a str, InvalidCCFinderSWResult>,
    {
        let result_parser = many1(alt((
            map(
                |i| self.parse_file_description(i),
                move |d| DataBlock::FileDescription(d),
            ),
            map(|i| self.parse_clone(i), move |c| DataBlock::Clone(c)),
            map(|i| self.parse_block(i), |_| DataBlock::Unknown),
            map(|i| self.parse_tag(i), |_| DataBlock::Unknown),
        )));
        map_res(result_parser, |blocks| {
            let mut file_description: Option<HashMap<(u32, u32), String>> = None;
            let mut clone: Option<Vec<CloneSet>> = None;
            for b in blocks {
                match b {
                    DataBlock::FileDescription(d) => {
                        if file_description.is_some() {
                            return Err(InvalidCCFinderSWResult::new(
                                "Duplicated file description blocks.",
                            ));
                        } else {
                            file_description = Some(d);
                        }
                    }
                    DataBlock::Clone(c) => {
                        if clone.is_some() {
                            return Err(InvalidCCFinderSWResult::new("Duplicated clone blocks."));
                        } else {
                            clone = Some(c);
                        }
                    }
                    DataBlock::Unknown => {}
                }
            }
            if file_description.is_none() || clone.is_none() {
                Err(InvalidCCFinderSWResult::new(
                    "Missing mandatory result blocks.",
                ))
            } else {
                Ok(ParsedResult::new(file_description.unwrap(), clone.unwrap()))
            }
        })(input)
    }

    pub fn new() -> Self {
        ResultParser {}
    }
}

#[cfg(test)]
mod test {
    use crate::clone_pair::{ClonePair, CodePosition, CodeSlice};
    use crate::runner::ccfindersw::parser::{CloneSet, ParsedResult, ResultParser, SetElement};
    use nom::lib::std::collections::HashMap;

    #[test]
    fn test_parse_digits() {
        let data = " 42 ";
        let parser = ResultParser::new();
        let res = parser.parse_digits::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        assert_eq!(res, 42);
    }

    #[test]
    fn test_parse_string() {
        let data = " /foo/bar/example.txt\r\n";
        let parser = ResultParser::new();
        let res = parser.parse_string::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        assert_eq!(res, "/foo/bar/example.txt");
    }

    #[test]
    fn test_parse_preceding_whitespace_0() {
        let data = " \t\r\naaa";
        let parser = ResultParser::new();
        let res = parser.parse_preceding_whitespace::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.clone().err().unwrap());
        }
        assert!(res.is_ok());
    }

    #[test]
    fn test_parse_preceding_whitespace_1() {
        let data = " \t\r\n";
        let parser = ResultParser::new();
        let res = parser.parse_preceding_whitespace::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.clone().err().unwrap());
        }
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_preceding_whitespace_2() {
        let data = "\t  \n";
        let parser = ResultParser::new();
        let res = parser.parse_preceding_whitespace::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.clone().err().unwrap());
        }
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_position() {
        let data = " 1,2,3 ";
        let parser = ResultParser::new();
        let res = parser.parse_position::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        let expected = CodePosition::new(1, 2);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_file_number() {
        let data = " 0.12";
        let parser = ResultParser::new();
        let res = parser.parse_file_number::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        assert_eq!(res, (0, 12));
    }

    #[test]
    fn test_parse_set_element() {
        let data = " 0.0     20,40,150       30,0,189        81\n";
        let parser = ResultParser::new();
        let res = parser.parse_set_element::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        let expected = SetElement::new(
            (0, 0),
            CodePosition::new(20, 40),
            CodePosition::new(30, 0),
            81,
        );
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_set() {
        let data = "\n#begin{set}
0.0     20,40,150       30,0,189        81
0.1     130,40,656      141,4,692       81
#end{set}\n";
        let parser = ResultParser::new();
        let res = parser.parse_set::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        let expected = vec![
            SetElement::new(
                (0, 0),
                CodePosition::new(20, 40),
                CodePosition::new(30, 0),
                81,
            ),
            SetElement::new(
                (0, 1),
                CodePosition::new(130, 40),
                CodePosition::new(141, 4),
                81,
            ),
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_parse_clone() {
        let data = "#begin{clone}
#begin{set}
0.0     20,40,150       30,0,189        81
0.1     130,40,656      141,4,692       81
#end{set}
#begin{set}
0.0     50,0,200       60,86,255        81
0.1     10,2,23      23,10,78       81
#end{set}
#end{clone}";
        let parser = ResultParser::new();
        let res = parser.parse_clone::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        let expected = vec![
            SetElement::new(
                (0, 0),
                CodePosition::new(20, 40),
                CodePosition::new(30, 0),
                81,
            ),
            SetElement::new(
                (0, 1),
                CodePosition::new(130, 40),
                CodePosition::new(141, 4),
                81,
            ),
        ];
        assert_eq!(res[0].elements, expected);
        let expected = vec![
            SetElement::new(
                (0, 0),
                CodePosition::new(50, 0),
                CodePosition::new(60, 86),
                81,
            ),
            SetElement::new(
                (0, 1),
                CodePosition::new(10, 2),
                CodePosition::new(23, 10),
                81,
            ),
        ];
        assert_eq!(res[1].elements, expected);
    }

    #[test]
    fn test_parse_file_description() {
        let data = "\n#begin{file description}
0.0     100     444     /tmp/.foo/src/Example.ino
0.1     250     1202    /tmp/.foo/src/MyProject.ino
#end{file description}\n";
        let parser = ResultParser::new();
        let res = parser.parse_file_description::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        assert!(res.contains_key(&(0 as u32, 0 as u32)));
        assert_eq!(
            res.get(&(0 as u32, 0 as u32)).unwrap(),
            "/tmp/.foo/src/Example.ino"
        );
        assert!(res.contains_key(&(0 as u32, 1 as u32)));
        assert_eq!(
            res.get(&(0 as u32, 1 as u32)).unwrap(),
            "/tmp/.foo/src/MyProject.ino"
        );
    }

    #[test]
    fn test_parse_tag_0() {
        let data = "\n#option: -c wfg\n";
        let parser = ResultParser::new();
        let res = parser.parse_tag::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
    }

    #[test]
    fn test_parse_tag_1() {
        let data = "#format: classwise\n";
        let parser = ResultParser::new();
        let res = parser.parse_tag::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
    }

    #[test]
    fn test_parse_block() {
        let data = "\n#begin{syntax error}
#end{syntax error}\n";
        let parser = ResultParser::new();
        let res = parser.parse_block::<nom::error::VerboseError<&str>>(&data);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
    }

    #[test]
    fn test_parse_result() {
        let result = "#format: classwise
#langspec: CPP
#option: -b 50
#option: -e char
#option: -k 30
#option: -r abdfikmnpstuv
#option: -c wfg
#option: -y
#begin{file description}
0.0     100     444     /tmp/.foo/src/Example.ino
0.1     250     1202    /tmp/.foo/src/MyProject.ino
#end{file description}
#begin{syntax error}
#end{syntax error}
#begin{clone}
#begin{set}
0.0     20,40,150       30,0,189        81
0.1     130,40,656      141,4,692       81
#end{set}
#end{clone}
";
        let parser = ResultParser::new();
        let res = parser.parse_result::<nom::error::VerboseError<&str>>(&result);
        if res.is_err() {
            eprintln!("Parse failed: {}", res.err().unwrap());
            assert!(false);
            return;
        }
        let (_left, res) = res.unwrap();
        let expected = ParsedResult::new(
            [
                ((0, 0), String::from("/tmp/.foo/src/Example.ino")),
                ((0, 1), String::from("/tmp/.foo/src/MyProject.ino")),
            ]
            .iter()
            .cloned()
            .collect(),
            [CloneSet::new(
                [
                    SetElement::new(
                        (0, 0),
                        CodePosition::new(20, 40),
                        CodePosition::new(30, 0),
                        81,
                    ),
                    SetElement::new(
                        (0, 1),
                        CodePosition::new(130, 40),
                        CodePosition::new(141, 4),
                        81,
                    ),
                ]
                .to_vec(),
            )]
            .to_vec(),
        );
        assert_eq!(res, expected);
    }
}
