use std::collections::BTreeMap;
use std::fmt::Debug;

pub fn diff_mats<T: PartialEq + Clone + Debug>(mats: &BTreeMap<&'static str, T>,
                                               new_mats: &BTreeMap<&'static str, T>)
                                               -> BTreeMap<&'static str, T> {
    let mut diff: BTreeMap<&str, T> = new_mats.clone(); 
    for (key, val ) in new_mats.iter() {
        let old_mat = &mats.get(key);
        if old_mat.is_some() && &old_mat.unwrap() == &val {
            diff.remove(key);
        }
    }
    diff
}

pub struct Output {
    last_msg: String
}
impl Output {
    pub fn new() -> Output {
        Output{last_msg: "".to_string()}
    }
    pub fn print_if_new (&mut self,  msg: String) {
        if msg != self.last_msg {
            println!("{}", msg);
            self.last_msg = msg;
        }
    }

}
