use anyhow::{Context, Result};

pub fn transpile(s: &str) -> Result<String> {
  print(parse(s)?)
}

fn parse(s: &str) -> Result<syn::File> {
  syn::parse_str(s).with_context(|| "Parse error")
}

fn print(file: syn::File) -> Result<String> {
  Ok(
    file
      .items
      .iter()
      .map(|item| {
        Ok(match item {
          syn::Item::Use(_) => None,
          syn::Item::Fn(fun) => Some(format!(
            "{} {} ({}) {{\n  {}\n}}",
            match &fun.sig.output {
              syn::ReturnType::Default => "void".to_string(),
              syn::ReturnType::Type(_, x) => cp(x),
            },
            fun.sig.ident,
            fun
              .sig
              .inputs
              .iter()
              .map(|param| Ok(match param {
                syn::FnArg::Typed(syn::PatType { ty, pat, .. }) =>
                  format!("{} {}", cp(ty), cp(pat)),
                _ => anyhow::bail!("Unsupported argument type"),
              }))
              .collect::<Result<Vec<_>>>()?
              .join(", "),
            {
              &fun
                .block
                .stmts
                .iter()
                .map(|statement| {
                  Ok(match statement {
                    syn::Stmt::Expr(x) => format!("return {};", cp(&x)),
                    syn::Stmt::Local(x) => match &x.pat {
                      syn::Pat::Type(syn::PatType { ty, pat, .. }) => {
                        format!(
                          "{} {} {};",
                          cp(&ty),
                          cp(&pat),
                          match &x.init {
                            Some((_, expression)) => format!("= {}", cp(&expression)),
                            None => "".to_string(),
                          }
                        )
                      }
                      _ => anyhow::bail!("Assignment is missing type"),
                    },
                    _ => cp(&statement),
                  })
                })
                .collect::<Result<Vec<_>>>()?
                .join("\n  ")
            },
          )),
          x => Some(pp(x)),
        })
      })
      .collect::<Result<Vec<_>>>()?
      .into_iter()
      .filter_map(|x| x)
      .collect::<Vec<_>>()
      .join("\n"),
  )
}

fn pp<T>(x: T) -> String
where
  T: std::fmt::Debug,
{
  format!("{:#?}", x)
}

fn cp<T>(x: &T) -> String
where
  T: quote::ToTokens,
{
  quote::quote!(#x).to_string()
}

#[test]
fn test() -> Result<()> {
  let shader_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shader.rs");
  let shader = std::fs::read_to_string(shader_path).unwrap();
  let f = parse(&shader)?;
  println!("{}", print(f)?);
  Ok(())
  // assert_eq!(parse(&shader), "")
}

// use nom::branch::alt;
// use nom::bytes::complete::take_until;
// use nom::character::complete::alphanumeric1;
// use nom::character::complete::char;
// use nom::character::complete::multispace0 as ws;
// use nom::combinator::map;
// use nom::combinator::opt;
// use nom::error::ParseError;
// use nom::sequence::pair;
// use nom::sequence::preceded;
// use nom::Compare;
// use nom::FindSubstring;
// use nom::InputLength;
// use nom::InputTake;
// use nom::Parser;
// use nom::{
//   bytes::complete::{tag, take_while_m_n},
//   combinator::map_res,
//   sequence::tuple,
//   IResult,
// };

// #[macro_export]
// macro_rules! join {
//     ($($es:expr),*) => {
//       |mut input|{
//         let mut acc = String::new();
//         $(
//           let (rest_input, res) = $es.parse(input)?;
//           add_to_string(&mut acc, res);
//           input = rest_input;
//         )*
//         Ok((input, acc))
//       }
//     };
// }

// fn add_to_string<S: AsRef<str>>(s: &mut String, added: S) {
//   s.push_str(added.as_ref())
// }

// type Res<'a> = IResult<&'a str, String>;
// type ResR<'a> = IResult<&'a str, &'a str>;

// fn parse(input: &str) -> Res {
//   preceded(ignore(tuple((tag("use"), take_until_and(";"), ws))), fn_def)(input)
//   // ignore(tuple((tag("use"), take_until_and(";"), ws)))(input)
//   // parameter_list(input)
// }

// #[test]
// fn test() {
//   let shader_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/shader.rs");
//   let shader = std::fs::read_to_string(shader_path).unwrap();
//   assert_eq!(parse(&shader).unwrap().1, "")
// }

// // pub fn pixel_color(coordinates: Float2) -> Float4 {
// //   float4(coordinates.x, coordinates.y, 0.0, 1.0)
// // }

// #[macro_export]
// macro_rules! form {
//   (
//     ($format_string:literal, $($token_name:ident),*),
//     $(
//       $token_pat:pat = $es:expr
//     ),*$(,)?
//   ) => {
//     |mut input|{
//       $(
//         let (rest_input, $token_pat) = $es.parse(input)?;
//         input = rest_input;
//       )*
//       Ok((input, format!($format_string, $($token_name),*)))
//     }
//   };
// }

// fn fn_def(input: &str) -> Res {
//   form!(
//     ("{} {} {}", ret_type, fn_name, params),
//     _ = tuple((opt(tag("pub")), ws, tag("fn"), ws)),
//     fn_name = name,
//     params = parameter_list,
//     ret_type = name,
//   )(input)
// }

// fn name(input: &str) -> ResR {
//   alt((alphanumeric1, tag("_")))(input)
// }

// // fn token<'a>(t: &'a str) -> impl FnMut(&'a str) -> ResR {
// //   tag(t)
// // }

// // pub fn alloc<'a, E, F>(p: F) -> impl FnMut(&'a str) -> IResult<&'a str, String, E>
// // where
// //   F: Parser<&'a str, &'a str, E>,
// // {
// //   map(p, String::from)
// // }

// fn parameter_list(input: &str) -> Res {
//   join!(tag("("), name, tag(":"), name, tag(")"))(input)
// }

// // fn parse(input: &str) -> IResult<&str, &str> {
// //   let (input, _) = tag("use", 1)(input)?;
// //   let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

// //   Ok((input, Color { red, green, blue }))
// // }

// // // first implement the basic parsers
// // let method = take_while1(is_alpha);
// // let space = take_while1(|c| c == ' ');
// // let url = take_while1(|c| c!= ' ');
// // let is_version = |c| c >= b'0' && c <= b'9' || c == b'.';
// // let http = tag("HTTP/");
// // let version = take_while1(is_version);
// // let line_ending = tag("\r\n");

// // // combine http and version to extract the version string
// // // preceded will return the result of the second parser
// // // if both succeed
// // let http_version = preceded(http, version);

// // // combine all previous parsers in one function
// // fn request_line(i: &[u8]) -> IResult<&[u8], Request> {

// //   // tuple takes as argument a tuple of parsers and will return
// //   // a tuple of their results
// //   let (input, (method, _, url, _, version, _)) =
// //     tuple((method, space, url, space, http_version, line_ending))(i)?;

// //   Ok((input, Request { method, url, version }))
// // }

// pub fn take_until_and<T, Input, Error: ParseError<Input>>(
//   t: T,
// ) -> impl FnMut(Input) -> IResult<Input, (Input, Input), Error>
// where
//   Input: InputTake + FindSubstring<T> + Compare<T>,
//   T: InputLength + Clone + Copy,
// {
//   pair(take_until(t), tag(t))
// }

// fn ignore<'a, O1, E, F>(mut p: F) -> impl FnMut(&'a str) -> IResult<&'a str, &str, E>
// where
//   F: Parser<&'a str, O1, E>,
// {
//   move |input| {
//     let (input, _result) = p.parse(input)?;
//     Ok((input, ""))
//   }
// }
