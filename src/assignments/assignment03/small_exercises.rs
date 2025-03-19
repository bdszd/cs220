//! Small problems.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Sun => DayOfWeek::Mon,
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        DayOfWeek::Fri => DayOfWeek::Mon,
        DayOfWeek::Sat => DayOfWeek::Mon,
    }
}

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```ignore
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```ignore
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    let len = values.len();
    let mut temp = values.clone();
    temp.sort();
    if len > 0 {
        if len / 2 == 0 {
            Some(temp[(len + 1) / 2 - 1])
        } else {
            Some(temp[len / 2])
        }
    } else {
        None
    }
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash
/// map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    let len = values.len();
    let mut prob = HashMap::new();
    if len > 0 {
        for &item in &values {
            *prob.entry(item).or_insert(1) += 1;
        }
        let mut mode = values[0];
        let mut count = 0;
        for (&k, &v) in &prob {
            if v > count || (v == count && k < mode) {
                mode = k;
                count = v;
            }
        }
        Some(mode)
    } else {
        None
    }
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig
/// Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end
///    of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    if vowels.contains(&input.chars().next().unwrap()) {
        format!("{}hay", &input)
    } else {
        let mut ind = 0;
        for (i, c) in input.chars().enumerate() {
            if vowels.contains(&c) {
                ind = i;
                break;
            }
        }

        let (first, last) = input.split_at(ind);
        format!("{}{}ay", last, first)
    }
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```ignore
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```ignore
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add {person} to {department}", "Remove {person} from {department}",
///   and "Move {person} from {department} to {department}".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut ret: HashMap<String, HashSet<String>> = HashMap::new();
    for command in commands {
        let part: Vec<&str> = command.split_whitespace().collect();
        match part.as_slice() {
            ["Add", person, "to", department] => {
                let _ = ret
                    .entry(department.to_string())
                    .or_default()
                    .insert(person.to_string());
            }
            ["Remove", person, "from", department] => {
                if let Some(set) = ret.get_mut(*department) {
                    let _ = set.remove(*person);
                    if set.is_empty() {
                        let _unused = ret.remove(*department);
                    }
                }
            }
            ["Move", person, "from", department_from, "to", department_to] => {
                if let Some(set1) = ret.get_mut(*department_from) {
                    let _ = set1.remove(*person);
                    if set1.is_empty() {
                        let _unused = ret.remove(*department_from);
                    }
                    let _ = ret
                        .entry(department_to.to_string())
                        .or_default()
                        .insert(person.to_string());
                }
            }
            _ => {}
        }
    }
    ret
}

/// Events in a text editor.
#[derive(Debug)]
pub enum TypeEvent {
    /// A character is typed.
    Type(char),
    /// The last character is removed.
    Backspace,
    /// The whole string is copied to the clipboard.
    Copy,
    /// The string in the clipboard is appended.
    Paste,
}

/// Starting from an empty string and an empty clipboard,
/// processes the given `events` in order and returns the resulting string.
///
/// See the test function `test_editor` for examples.
pub fn use_editor(events: Vec<TypeEvent>) -> String {
    let mut ret = String::new();
    let mut clip = String::new();
    for item in events.iter() {
        match item {
            TypeEvent::Type(v) => ret.push(*v),
            TypeEvent::Backspace => {
                if !ret.is_empty() {
                    _ = ret.pop();
                }
            }
            TypeEvent::Copy => clip = ret.clone(),
            TypeEvent::Paste => {
                if !clip.is_empty() {
                    use ::std::ops::Add;
                    ret = ret.add(&clip);
                }
            }
        }
    }
    ret
}
