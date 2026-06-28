use rand::prelude::IndexedRandom;
use std::io::Write;
use std::io;

/*

README:

make a text file with questions in the following format in the appropriate directory for the program, named "questions.txt":

questionorvocab1:answerordefinition1
questionorvocab2:answerordefinition2
questionorvocab3:answerordefinition3
etc:etc

make sure question and answer are seperated by ":" and do not have any other ":" in the line besides that.

TODO:
add % remaining questions and % correct at finish.

*/

fn main() {
    let mut running = true;
    let mut questions = vec![];
    if let Ok(contents) = std::fs::read_to_string("questions.txt"){
        let contents = contents.lines();
        let contents = contents.collect::<Vec<&str>>();
        for i in contents {
            let line = i.trim();
            let line = line.split(":");
            let line = line.collect::<Vec<&str>>();
            let question = line[0];
            let answer = line[1];
            questions.push((question.to_string(), answer.to_string()));
        }
    }
    println!("Enter the word quit instead of an answer to exit.");
    let mut answer_pool = questions.clone();
    while running {
        if answer_pool.len() == 0 {
            println!("All questions answered correctly!");
            running = false;
            break;
        }
        let mut _choices = vec![];
        let mut _questions = questions.clone();
        let mut rng = rand::rng();
        let max_questions = if questions.len() > 4 {
            4
        } else {
            questions.len()
        };
        let question = answer_pool.choose(&mut rng).unwrap();
        let q = &question.0;
        let a = &question.1;
        let mut n = 0;
        let question_index = _questions.iter().position(|x| *x == (q.to_string(), a.to_string())).unwrap();
        _questions.remove(question_index);
        _choices.push(question.clone());
        for i in 0..max_questions-1 {

            let choice = _questions.choose(&mut rng).unwrap().clone();
            let index = _questions.iter().position(|x| *x == choice).unwrap();
            _questions.remove(index);
            _choices.push(choice);
        }
        let mut choices = vec![];
        for i in 1.._choices.len()+1 {
            let choice = _choices.choose(&mut rng).unwrap().clone();
            choices.push((i.to_string(), choice.0.clone(), choice.1.clone()));
            let index = _choices.iter().position(|x| *x == choice).unwrap();
            _choices.remove(index);
            if choice == *question {
                n += i;
            }
        }
        let n = n.to_string();
        let mut final_string = format!("\r\nQuestion: {}\r\nAnswers:", q);
        for i in 0..choices.len() {
            let ans = if choices[i].2 == *a {
                if choices[i].0 != n {
                    "(duplicate, do not pick)"
                }
                else{
                    &choices[i].2
                }
            } else {
                &choices[i].2
            };
            final_string += &format!("\r\n{}: {}", choices[i].0, ans);
        }
        println!("{}", final_string);
        println!("\r\nEnter the number of the correct answer and press enter...\r\n");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input.\r\n");
        let input = input.trim();
        let input = input.to_lowercase();
        let input = input.as_str();
        if input == "quit" {
            break;
        }
        if input == n {
            println!("\r\nThat is correct!!! Press any button to continue...");
            let question_index = answer_pool.iter().position(|x| *x == *(question)).unwrap();
            answer_pool.remove(question_index);
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
        else {
            println!("\r\nThat is incorrect. Press any button to continue...");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
        }
    }
}
