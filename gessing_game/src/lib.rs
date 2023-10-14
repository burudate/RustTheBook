use std::cmp::Ordering;

pub fn judge(target: &u32, input: &u32) -> (String, bool) {
    let mut ret = (String::from(""), false);
    match input.cmp(target) {
        Ordering::Less => ret.0 = String::from("小さい"),
        Ordering::Greater => ret.0 = String::from("大きい"),
        Ordering::Equal =>{
            ret.0 = String::from("あたり");
            ret.1 = true;
        },
    }
    ret
}


#[cfg(test)]
mod test {
    #[test]
    fn case_less() {
        let tgt = 2;
        let inp = 1;
        let ret = crate::judge(&tgt, &inp);
        assert_eq!("小さい", ret.0);
        assert!(!ret.1);
    }

    #[test]
   fn case_greater() {
        let tgt = 1;
        let inp = 2;
        let ret = crate::judge(&tgt, &inp);
        assert_eq!("大きい", ret.0);
        assert!(!ret.1);
    }

    #[test]
    fn case_equal() {
        let tgt = 1;
        let inp = 1;
        let ret = crate::judge(&tgt, &inp);
        assert_eq!("あたり", ret.0);
        assert!(ret.1);
    }
}
