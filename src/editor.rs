use std::collections::HashMap;
use serde::{Deserialize, Serialize};
// use bincode;
use AsgoreCore;
use std::fs;
use colored::Colorize;
use std::io::{self, Write};
use lazy_static::lazy_static;
use terminal_size::{Width, Height, terminal_size};

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum MyDataType {
    Isize,
    String,
}
impl MyDataType {
    pub fn get_color(self) -> [u8; 3] {
        match self {
            MyDataType::Isize => [68, 139, 211],
            MyDataType::String => [38, 209, 111],
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub enum MyValueType {
    Isize { value: isize },
    String { value: String },
}
impl MyValueType {
    pub fn try_to_isize(self) -> isize {
        if let MyValueType::Isize { value } = self {
            value
        } else {
            isize::default()
        }
    }
    
    pub fn try_to_String(self) -> String {
        if let MyValueType::String { value } = self {
            value
        } else {
            String::default()
        }
    }
    
    pub fn parse_value(&self, str_value: &str) -> MyValueType {
        match self {
            MyValueType::Isize { value } => MyValueType::Isize { value: str_value.parse::<isize>().unwrap_or(0) },
            MyValueType::String { value } => MyValueType::String { value: AsgoreCore::fix_escape_chars(str_value) },
        }
    }

    pub fn get_data_type(&self) -> MyDataType {
        match self {
            MyValueType::Isize { value } => MyDataType::Isize,
            MyValueType::String { value } => MyDataType::String,
        }
    }

    pub fn get_print_text(&self) -> String {
        match self {
            MyValueType::Isize { value } => value.to_string(),
            MyValueType::String { value } => value.to_string(),
            // format!("{:?}", value.to_string())
        }
    }
}

lazy_static! {
	static ref NODES_MAP: HashMap<&'static str, MyNodeTemplate> = [
        ("New number", MyNodeTemplate::NewNumber),
        ("Repeat string", MyNodeTemplate::RepeatString),
	].iter().cloned().collect();
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub enum MyNodeTemplate {
    NewNumber,
    RepeatString,
}
impl MyNodeTemplate {
    pub fn new(self) -> Node {
        let mut node: Node;
        match self {
            MyNodeTemplate::NewNumber => {
                node = Node::new(self, "New number", MyDataType::Isize);
                node.add_input("number", MyValueType::Isize { value: isize::default() });
            }
            MyNodeTemplate::RepeatString => {
                node = Node::new(self, "Repeat string", MyDataType::String);
                node.add_input("string", MyValueType::String { value: String::default() });
                node.add_input("number", MyValueType::Isize { value: isize::default() });
            }
        };
        node
    }

    pub fn get_node_by_title(node_title: &str) -> Option<MyNodeTemplate> {
        NODES_MAP.get(node_title).cloned()
    }

    pub fn get_node_titles() -> Vec<String> {
        NODES_MAP.keys().map(|x| x.to_string()).collect()
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Input {
    connected_id: usize,
    input_type: MyDataType,
    color: [u8; 3],
    label: String,
    pub value: MyValueType,
}
impl Input {
    pub fn new(label: &str, value: MyValueType) -> Self {
        let input_type = value.get_data_type();
        Self {
            connected_id: 0,
            input_type: input_type,
            color: input_type.get_color(),
            label: label.to_string(),
            value: value,
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Output {
    connected_ids: Vec<usize>,
    output_type: MyDataType,
    color: [u8; 3],
}
impl Output {
    pub fn new(output_type: MyDataType) -> Self {
        Self {
            connected_ids: Vec::new(),
            output_type: output_type,
            color: output_type.get_color(),
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize)]
pub struct Node {
    id: usize,
    title: String,
    node_type: MyNodeTemplate,
    pub inputs: Vec<Input>,
    output: Output,
    pub x: usize,
    pub y: usize,
    pub w: usize,
}
impl Node {
    pub fn new(node_type: MyNodeTemplate, title: &str, output_type: MyDataType) -> Self {
        Self {
            id: 0,
            title: title.to_string(),
            node_type: node_type,
            inputs: Vec::new(),
            output: Output::new(output_type),
            x: 0,
            y: 0,
            w: 0,
        }
    }

    pub fn add_input(&mut self, label: &str, defaul: MyValueType) {
        self.inputs.push(Input::new(label, defaul));
    }
}

#[derive(Serialize, Deserialize)]
pub struct Editor {
    nodes: HashMap<usize, Node>,
    outputs_cache: HashMap<usize, MyValueType>,
    current_id: usize,
    auto_save: bool,
    w: usize,
    h: usize
}
impl Editor {
    pub fn new() -> Self { // w: usize, h: usize
        Self {
            nodes: HashMap::new(),
            outputs_cache: HashMap::new(),
            current_id: 0,
            auto_save: false,
            w: 0,
            h: 0
        }
    }

    pub fn load(&self, path: &str) -> Self {
        let binary_data: Vec<u8> = match fs::read(path) {
            Ok(f) => f,
            Err(_) => {
                println!("the file can not be loaded.");
                Vec::new()
            },
        };
        if binary_data.is_empty() {
            return Self::new();
        }
        match bincode::deserialize(&binary_data) {
            Ok(data) => data,
            Err(_) => {
                println!("the save file is curropted.");
                Self::new()
            },
        }
    }

    pub fn save(&self, path: &str) {
        let binary_data = bincode::serialize(&self).unwrap();
        AsgoreCore::try_write_byte_file(path, &binary_data);
    }

    pub fn enable_auto_save(&mut self) {
        self.auto_save = true;
    }

    pub fn disable_auto_save(&mut self) {
        self.auto_save = false;
    }

    pub fn add_node(&mut self, node: Node) {
        self.current_id += 1;
        let mut node = node;
        node.id = self.current_id;
        self.nodes.insert(node.id, node);
    }

    fn auto_save(&self) {
        if self.auto_save {
            self.save("auto_save.ane")
        }
    }

    fn are_types_matched(&self, from_node_id: usize, to_node_id: usize, input_index: usize) -> bool {
        self.nodes.get(&from_node_id).unwrap().output.output_type == self.nodes.get(&to_node_id).unwrap().inputs[input_index].input_type
    }

    fn is_safe_from_infinite_loop(&self, from_node_id: usize, to_node_id: usize, passed_nodes_ids: &Vec<usize>) -> bool {
        let from_node = self.nodes.get(&from_node_id).unwrap();
        let mut passed_nodes_ids = passed_nodes_ids.clone();
        passed_nodes_ids.push(to_node_id);
        for input in &from_node.inputs {
            if passed_nodes_ids.contains(&from_node_id) ||
                input.connected_id != 0 && !self.is_safe_from_infinite_loop(input.connected_id, from_node_id, &passed_nodes_ids) {
                return false
            }
        }
        true
    }

    pub fn connect_nodes(&mut self, from_node_id: usize, to_node_id: usize, input_index: usize) {
        if self.is_safe_from_infinite_loop(from_node_id, to_node_id, &Vec::new()) && self.are_types_matched(from_node_id, to_node_id, input_index) {
            self.nodes.get_mut(&to_node_id).unwrap().inputs[input_index].connected_id = from_node_id;
            self.nodes.get_mut(&from_node_id).unwrap().output.connected_ids.push(to_node_id);
        }
        self.auto_save();
    }

    pub fn remove_connection(&mut self, to_node_id: usize, input_index: usize) {
        self.nodes.get_mut(&to_node_id).unwrap().inputs[input_index].connected_id = 0;
        let from_node = self.nodes.get_mut(&to_node_id).unwrap();
        if let Some(index) = from_node.output.connected_ids.iter().position(|&x| x == to_node_id) {
            from_node.output.connected_ids.remove(index);
        }
        self.auto_save();
    }

    pub fn disconnect_node_to_remove(&mut self, node_id: usize) {
        let node = self.nodes.get(&node_id).unwrap();
        for from_node_input in node.inputs.clone() {
            if from_node_input.connected_id != 0 {
                let from_node = self.nodes.get_mut(&from_node_input.connected_id).unwrap();
                from_node.output.connected_ids.retain(|&x| x != node_id);
            }
        }
        let node = self.nodes.get(&node_id).unwrap();
        for to_node_id in node.output.connected_ids.clone() {
            let to_node = self.nodes.get_mut(&to_node_id).unwrap();
            for i in 0..to_node.inputs.len() {
                if to_node.inputs[i].connected_id == node_id {
                    to_node.inputs[i].connected_id = 0;
                    break;
                }
            }
        }
        self.auto_save();
    }

    pub fn remove_node(&mut self, node_id: usize) {
        self.disconnect_node_to_remove(node_id);
        self.nodes.remove(&node_id);
        // self.auto_save();
    }

    pub fn evaluate_node(&mut self, node_id: usize) -> MyValueType {
        self.outputs_cache = HashMap::new();
        self._evaluate_node(node_id)
    }

    fn _evaluate_node(&self, node_id: usize) -> MyValueType {
        let node = self.nodes.get(&node_id).unwrap();
        match node.node_type {
            MyNodeTemplate::NewNumber => {
                let _0 = self.evaluate_input(&node.inputs[0]).try_to_isize();
                MyValueType::Isize { value: _0 }
            }
            MyNodeTemplate::RepeatString => {
                let _0 = self.evaluate_input(&node.inputs[0]).try_to_String();
                let _1 = self.evaluate_input(&node.inputs[1]).try_to_isize();
                MyValueType::String { value: _0.repeat(_1 as usize) }
            }
        }
    }

    pub fn evaluate_input(&self, input: &Input) -> MyValueType {
        if input.connected_id == 0 {
            input.value.clone()
        }
        else {
            match self.outputs_cache.get(&input.connected_id) {
                Some(value) => value.clone(),
                _ => self._evaluate_node(input.connected_id),
            }
        }
    }
}

pub trait UI {
    fn clear_win(&self);
    // fn is_out_of_win(&self, x: usize, y: usize) -> bool;
    fn draw_rect(
        &self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    );
    fn draw_container(
        &self,
        title: String,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    );
    fn draw_textarea(&self, text: &str, x: usize, y: usize) -> (usize, usize);
    fn draw_textarea_cut_overflaw(&self, text: &str, x: usize, y: usize, w: usize, h: usize);
    fn draw_bordered_textarea (
        &self,
        text: &str,
        x: usize,
        y: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    ) -> (usize, usize);
    fn draw_bordered_textarea_cut_overflaw(
        &self,
        text: &str,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    );
    fn draw_node(
        &mut self,
        node: &mut Node,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char,
        input_socket: char,
        output_socket: char,
        default_value_display_length: usize
    );
    fn draw_nodes(
        &mut self,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char,
        input_socket: char,
        output_socket: char,
        default_value_display_length: usize
    );
    fn draw_connection(
        &self,
        from_node_w: usize,
        to_node_id: usize,
        input_index: usize,
        color: [u8; 3],
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        hedge: char,
        vedge: char
    );
    fn run(&mut self);
    fn draw(&mut self);
    fn events(&mut self) -> bool;
}
impl UI for Editor {
    #[cfg(target_os = "windows")]
    fn clear_win(&self) {
        std::process::Command::new("cmd")
            .arg("/c")
            .arg("cls")
            .status()
            .unwrap();
    }

    #[cfg(target_os = "unix")]
    fn clear_win(&self) {
        std::process::Command::new("clear")
            .status()
            .unwrap();
    }

    // fn is_out_of_win(&self, x: usize, y: usize) -> bool {
    //     x > self.w || y > self.h
    // }

    // fn is_out_of_bounds(&self, x: usize, y: usize) -> bool {
    //     x > self.w || y > self.h
    // }

    fn draw_rect(
        &self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    ) {
        print!("\x1B[{};{}H{}{}{}", y, x, tlcorner, AsgoreCore::repeat_char(tedge, w), trcorner);
        for i in 1..=h {
            print!("\x1B[{};{}H{}", y + i, x, ledge);
            print!("\x1B[{};{}H{}", y + i, x + w + 1, redge);
        }
        print!("\x1B[{};{}H{}{}{}", y + h + 1, x, dlcorner, AsgoreCore::repeat_char(dedge, w), drcorner);
    }

    fn draw_container(
        &self,
        title: String,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    ) {
        self.draw_rect(x, y, w, h, trcorner, tlcorner, drcorner, dlcorner, tedge, dedge, redge, ledge);
        print!("\x1B[{};{}H{}", y, x+1, (" ".to_owned() + &title + " ").chars().take(w).collect::<String>());
    }
    
    fn draw_textarea(&self, text: &str, x: usize, y: usize) -> (usize, usize) {
        let lines = text.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
        let longest = lines.iter().map(|x| x.chars().count()).max().unwrap_or(0);
        if longest == 0 {
            return (0, 0);
        }
        for (i, line) in lines.iter().enumerate() {
            print!("\x1B[{};{}H{}", y + i, x, line.to_owned() + &" ".repeat(longest - line.len()));
        }
        (longest, lines.len())
    }
    
    fn draw_textarea_cut_overflaw(&self, text: &str, x: usize, y: usize, w: usize, h: usize) {
        let lines = text.split("\n").map(|x| x.to_string()).collect::<Vec<String>>();
        let longest = lines.iter().map(|x| x.chars().count()).max().unwrap_or(0);
        if longest == 0 {
            return;
        }
        for i in 0..std::cmp::min(lines.len(), h) {
            let line = &lines[i];
            if line.chars().count() > w {
                let mut t = line.chars().take(std::cmp::max(w-1, 0)).map(|x| x.to_string()).collect::<String>();
                t += if longest > 1 {"-"} else {""};
                print!("\x1B[{};{}H{}", y + i, x, t);
            } else {
                print!("\x1B[{};{}H{}", y + i, x, line.to_owned() + &" ".repeat(w - line.len()));
            }
        }
    }

    fn draw_bordered_textarea(
        &self,
        text: &str,
        x: usize,
        y: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    ) -> (usize, usize) {
        let (w, h) = self.draw_textarea(text, x+1, y+1);
        self.draw_rect(x, y, w, h, trcorner, tlcorner, drcorner, dlcorner, tedge, dedge, redge, ledge);
        (w, h)
    }
    
    fn draw_bordered_textarea_cut_overflaw(
        &self,
        text: &str,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char
    ) {
        self.draw_textarea_cut_overflaw(text, x+1, y+1, w, h);
        self.draw_rect(x, y, w, h, trcorner, tlcorner, drcorner, dlcorner, tedge, dedge, redge, ledge);
    }

    fn draw_node(
        &mut self,
        node: &mut Node,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char,
        input_socket: char,
        output_socket: char,
        default_value_display_length: usize
    ) {
        let mut text = " [".to_owned() + &node.title + "] ";
        for input in &node.inputs {
            text += &("\n ".to_owned() + &input.label + &" ".repeat(default_value_display_length+2));
        }
        text += &("\n (id: ".to_owned() + &node.id.to_string() + ")");
        let (w, _) = self.draw_bordered_textarea(&text, node.x, node.y, trcorner, tlcorner, drcorner, dlcorner, tedge, dedge, redge, ledge);
        node.w = w;
        for (i, input) in node.inputs.iter().enumerate() {
            if input.connected_id == 0 {
                self.draw_textarea_cut_overflaw(&input.value.get_print_text(), node.x+w-default_value_display_length, node.y+2+i, default_value_display_length, 1);
            }
        }
        for (i, input) in node.inputs.iter().enumerate() {
            let input_color = input.color;
            print!("\x1B[{};{}H{}", node.y + 2 + i, node.x, input_socket.to_string().truecolor(input_color[0], input_color[1], input_color[2]));
            if node.inputs[i].connected_id != 0 {
                self.draw_connection(w, node.id, i, input_color, '┐', '┌', '┘', '└', '─', '│');
            }
        }
        let output_color = node.output.color;
        print!("\x1B[{};{}H{}", node.y + 2, node.x + w + 1, output_socket.to_string().truecolor(output_color[0], output_color[1], output_color[2]));
    }
    
    fn draw_nodes(
        &mut self,
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        tedge: char,
        dedge: char,
        redge: char,
        ledge: char,
        input_socket: char,
        output_socket: char,
        default_value_display_length: usize
    ) {
        for (_,  mut node) in self.nodes.clone() {
            self.draw_node(&mut node, trcorner, tlcorner, drcorner, dlcorner, tedge, dedge, redge, ledge, input_socket, output_socket, default_value_display_length);
        }
    }

    fn draw_connection(
        &self,
        from_node_w: usize,
        to_node_id: usize,
        input_index: usize,
        color: [u8; 3],
        trcorner: char,
        tlcorner: char,
        drcorner: char,
        dlcorner: char,
        hedge: char,
        vedge: char
    ) {
        let to_node = &self.nodes[&to_node_id];
        let from_node = &self.nodes[&to_node.inputs[input_index].connected_id];

        let mut start_pos_x = from_node.x + from_node_w - 1;
        let mut end_pos_x = to_node.x;
        let mut start_pos_y = from_node.y + 2;
        let mut end_pos_y = to_node.y + input_index + 2;
        
        if start_pos_x > end_pos_x {
            (start_pos_x, end_pos_x) = (end_pos_x + 1, start_pos_x + 1);
            (start_pos_y, end_pos_y) = (end_pos_y, start_pos_y);
        };

        let connection_len = end_pos_x - start_pos_x;
        let connection_half_len = connection_len / 2;

        if start_pos_y == end_pos_y {
            print!("\x1B[{};{}H{}", end_pos_y, start_pos_x + connection_half_len, hedge.to_string().truecolor(color[0], color[1], color[2]));
        } else if start_pos_y > end_pos_y {
            for i in (end_pos_y+1)..start_pos_y {
                print!("\x1B[{};{}H{}", i, start_pos_x + connection_half_len, vedge.to_string().truecolor(color[0], color[1], color[2]));
            }
            print!("\x1B[{};{}H{}", start_pos_y, start_pos_x + connection_half_len, drcorner.to_string().truecolor(color[0], color[1], color[2]));
            print!("\x1B[{};{}H{}", end_pos_y, start_pos_x + connection_half_len, tlcorner.to_string().truecolor(color[0], color[1], color[2]));
        } else {
            for i in (start_pos_y+1)..end_pos_y {
                print!("\x1B[{};{}H{}", i, start_pos_x + connection_half_len, vedge.to_string().truecolor(color[0], color[1], color[2]));
            }
            print!("\x1B[{};{}H{}", start_pos_y, start_pos_x + connection_half_len, trcorner.to_string().truecolor(color[0], color[1], color[2]));
            print!("\x1B[{};{}H{}", end_pos_y, start_pos_x + connection_half_len, dlcorner.to_string().truecolor(color[0], color[1], color[2]));
        }
        
        print!("\x1B[{};{}H{}", start_pos_y, start_pos_x, AsgoreCore::repeat_char(hedge, connection_half_len).truecolor(color[0], color[1], color[2]));
        print!("\x1B[{};{}H{}", end_pos_y, start_pos_x + connection_half_len + 1, AsgoreCore::repeat_char(hedge, connection_len - connection_half_len - 1).truecolor(color[0], color[1], color[2]));
    }

    fn draw(&mut self) {
        self.clear_win();
        let out_h = self.h/5;
        let editor_h = self.h - out_h -3;
        self.draw_container("Editor".to_string(), 1, 1, self.w, editor_h, '┐', '┌', '┘', '└', '─', '─', '│', '│');
        self.draw_nodes('╗', '╔', '╝', '╚', '─', '─', '║', '║', '╣', '╠', 5);
        self.draw_container("Output".to_string(), 1, editor_h + 3, self.w, out_h, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    
        print!("\x1B[{};{}H> ", self.h+5, 0);
        io::stdout().flush().unwrap();
    }

    fn events(&mut self) -> bool {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap_or(0);
        command = command.trim().to_owned();
        
        let out_h = self.h/5;
        let editor_h = self.h - out_h -3;
        if command.starts_with("calc_out ") {
            let node_id: usize = command[9..].parse().unwrap_or(0); // command[9..(command.len()-2)]
            if self.nodes.contains_key(&node_id) {
                let text = &self.evaluate_node(node_id).get_print_text();
                self.draw_textarea_cut_overflaw(text, 2, editor_h+4, self.w, out_h);
            }
        }
        else if command.starts_with("pos ") {
            let args: Vec<&str> = command[4..].split(" ").collect();
            if args.len() != 3 {
                return false;
            }
            let node_id = args[0].parse().unwrap_or(0);
            let x = args[1].parse().unwrap_or(0);
            let y = args[2].parse().unwrap_or(0);
            
            let mut node = self.nodes.get_mut(&node_id).unwrap();
            node.x = x;
            node.y = y;
        }
        else if command.starts_with("con ") {
            let args: Vec<&str> = command[4..].split(" ").collect();
            if args.len() != 3 {
                return false;
            }
            let from_node_id = args[0].parse().unwrap_or(0);
            let to_node_id = args[1].parse().unwrap_or(0);
            let input_index = args[2].parse().unwrap_or(0);
            
            let mut node = self.nodes.get_mut(&to_node_id).unwrap();

            if node.inputs[input_index].connected_id == 0 {
                self.connect_nodes(from_node_id, to_node_id, input_index);
            } else if node.inputs[input_index].connected_id == from_node_id {
                self.remove_connection(to_node_id, input_index);
            } else {
                self.remove_connection(to_node_id, input_index);
                self.connect_nodes(from_node_id, to_node_id, input_index);
            }
        }
        else if command.starts_with("set_val ") {
            let args: Vec<&str> = command[8..].split(" ").collect();
            if args.len() != 3 {
                return false;
            }
            let node_id = args[0].parse().unwrap_or(0);
            let input_index = args[1].parse().unwrap_or(0);
            
            let mut node = self.nodes.get_mut(&node_id).unwrap();
            node.inputs[input_index].value = node.inputs[input_index].value.parse_value(args[2]);
        }
        else if command.starts_with("add_node ") {
            let node_title = &command[9..];
            if let Some(node) = MyNodeTemplate::get_node_by_title(node_title) {
                let mut node = node.new();
                node.x = 2;
                node.y = 2;
                self.add_node(node);
            };
        }
        else if command.starts_with("del_node ") {
            if let Ok(node_id) = command[9..].parse() {
                self.remove_node(node_id);
            }
        }
        else if command.starts_with("save ") {
            let filepath = &(command[5..].to_string() + ".ane");
            self.save(filepath);
        }
        else if command.starts_with("load ") {
            let filepath = &(command[5..].to_string() + ".ane");
            *self = self.load(filepath);
        }
        else if command == "autosave on" {
            self.auto_save = true;
        }
        else if command == "autosave off" {
            self.auto_save = false;
        }
        else if command == "q" {
            return true;
        }
        else {
            println!("Unknown command, please try again.");
        }

        false
    }
    
    fn run(&mut self) {
        loop {
            let size = terminal_size();
            if let Some((Width(w), Height(h))) = size {
                self.w = w as usize - 2;
                self.h = h as usize - 2;
            }
            self.draw();

            if self.events() {
                break;
            }
        }
    }
}