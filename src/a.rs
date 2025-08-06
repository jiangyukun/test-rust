

fn main() {
    let mut index = 0;
    let next_word = || -> &str {
        index = index + 1;
    };
    let before_word = || {
        index = index - 1;
    };

    loop {
        if index >= result.len() {
            break;
        }
        a(next_word, before_word);
    }
}

fn a<F>(next_word: &F, before_word: &F)
where
    F: FnMut(),
{
}
