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
