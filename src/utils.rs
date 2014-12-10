extern crate ncurses;

pub enum Color{
	GREEN = 1,
	YELLOW = 2,
	WHITE = 3,
	CYAN = 4,
	MAGENTA = 5,
    BLUE = 6,
    RED = 7
}

pub fn format_middle(val: String, width: uint) -> String {
    let len = val.len();
    let mut res: Vec<String> = vec![];
    if len < width {
        let end = (width - len) / 2;
        let start = width - len - end;
        res.push(String::from_char(start, ' '));
        res.push(val);
        res.push(String::from_char(end, ' '));
    } else {
        res.push(val.to_string());
    }
    res.concat()
}
