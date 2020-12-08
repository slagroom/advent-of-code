use lazy_static::lazy_static;
use regex::Regex;
use std::io::prelude::*;
use std::io;
use std::str::FromStr;

trait PasswordRule
{
    fn eval(&self, password: &str) -> bool;
}

struct CharCountRule
{
    c: char,
    min: usize,
    max: usize,
}

impl CharCountRule {

    fn new(s: &str) -> Option<CharCountRule>
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+)\s+(?P<char>.)$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => 
            {
                println!("failed to parse rule: {}", s);
                return None;
            }
            Some(x) => x,
        };

        return Some(CharCountRule
        {
            c: captures.name("char").map(|c| c.as_str()).unwrap().chars().collect::<Vec<_>>()[0],
            min: usize::from_str(captures.name("min").unwrap().as_str()).unwrap(),
            max: usize::from_str(captures.name("max").unwrap().as_str()).unwrap(),
        });
    }
}

impl PasswordRule for CharCountRule
{
    fn eval(&self, password: &str) -> bool
    {
        let count = password.chars().filter(|c| *c == self.c).count();
        return self.min <= count && count <= self.max
    }
}

#[test]
fn char_count_rule_new() {

    let none = CharCountRule::new("1-2 f: password");
    assert!(none.is_none());

    let rule = CharCountRule::new("23-456 g").unwrap();
    assert_eq!(rule.c, 'g');
    assert_eq!(rule.min, 23);
    assert_eq!(rule.max, 456);
}

#[test]
fn char_count_rule_eval() {

    let rule = CharCountRule::new("2-4 g").unwrap();

    assert!(!rule.eval("goole"));
    assert!(rule.eval("google"));
    assert!(rule.eval("googgle"));
    assert!(rule.eval("googggle"));
    assert!(!rule.eval("googgggle"));
}

struct CharPositionRule
{
    c: char,
    a: usize,
    b: usize,
}

impl CharPositionRule
{
    fn new(s: &str) -> Option<CharPositionRule>
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(?P<a>\d+)-(?P<b>\d+)\s+(?P<char>.)$").unwrap();
        }

        let captures = match RE.captures(s) {
            None => 
            {
                println!("failed to parse {}", s);
                return None;
            },
            Some(x) => x,
        };

        return Some(CharPositionRule
        {
            c: captures.name("char").map(|c| c.as_str()).unwrap().chars().collect::<Vec<_>>()[0],
            a: usize::from_str(captures.name("a").unwrap().as_str()).unwrap(),
            b: usize::from_str(captures.name("b").unwrap().as_str()).unwrap(),
        });
    }
}

impl PasswordRule for CharPositionRule
{
    fn eval(&self, password: &str) -> bool
    {
        let pwd = password.chars().collect::<Vec<_>>();
        return [self.a - 1, self.b - 1].iter()
            .filter(|i| pwd.len() > **i && pwd[**i] == self.c)
            .count() == 1;
    }
}

#[test]
fn char_position_rule_new() {

    let none = CharPositionRule::new("1-2 f: password");
    assert!(none.is_none());

    let rule = CharPositionRule::new("23-456 g").unwrap();
    assert_eq!(rule.c, 'g');
    assert_eq!(rule.a, 23);
    assert_eq!(rule.b, 456);
}

#[test] 
fn char_position_rule_eval() {

    let rule = CharPositionRule::new("1-3 g").unwrap();

    assert!(!rule.eval("goole"));
    assert!(rule.eval("ogoole"));
    assert!(rule.eval("ooogoole"));
    assert!(!rule.eval("ogogole"));
}

struct PasswordEntry<'a>
{
    password: &'a str,
    rules: Vec<Box<dyn PasswordRule>>,
}

impl PasswordEntry<'_>
{
    fn parse<'a>(s: &'a str, parse_rule: impl Fn(&'a str) -> Box<dyn PasswordRule>) -> Option<PasswordEntry<'a>>
    {
        let [rule, password] = match s.split(":").map(|t| t.trim()).collect::<Vec<_>>()[..]
        {
            [x, y] => [x, y],
            _ => return None,
        };

        return Some(PasswordEntry
        {
            password: password,
            rules: vec![parse_rule(rule)],
        });
    }

    fn valid(&self) -> bool
    {
        return self.rules.iter().filter(|r| !r.eval(self.password)).count() == 0;
    }
}

fn parse_char_count_rule(rule: &str) -> Box::<dyn PasswordRule>
{
    return Box::new(CharCountRule::new(rule).unwrap());
}

fn parse_char_position_rule(rule: &str) -> Box::<dyn PasswordRule>
{
    return Box::new(CharPositionRule::new(rule).unwrap());
}

fn main()
{
    let input = read_input();

    println!("part 1: {}", input.iter()
        .map(|line| PasswordEntry::parse(&line, parse_char_count_rule).unwrap())
        .filter(|entry| entry.valid())
        .count());

    println!("part 2: {}", input.iter()
        .map(|line| PasswordEntry::parse(&line, parse_char_position_rule).unwrap())
        .filter(|entry| entry.valid())
        .count());
}

fn read_input() -> Vec<String>
{
    return io::stdin().lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
}
