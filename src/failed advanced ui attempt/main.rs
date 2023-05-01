mod editor;
use editor::*;

// const TITLE: &str = "Asgore Terminal Node Editor";

fn main() {
    let mut node1 = MyNodeTemplate::RepeatString.new();
    node1.inputs[0].value = MyValueType::String { value: "fdgasd".to_string() };
    node1.x = 70;
    node1.y = 15;
    let mut node2 = MyNodeTemplate::NewNumber.new();
    node2.inputs[0].value = MyValueType::Isize { value: 4 };
    node2.x = 30;
    node2.y = 16;
    let mut node3 = MyNodeTemplate::NewNumber.new();
    node3.inputs[0].value = MyValueType::Isize { value: 4 };
    node3.x = 25;
    node3.y = 6;

    let mut editor = Editor::new();
    editor.add_node(node1);
    editor.add_node(node2);
    editor.add_node(node3);
    editor.connect_nodes(2, 1, 1);

    // editor.clear_win();
    // editor.draw_container("Editor".to_string(), 1, 1, 100, 20, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    // editor.draw_rect(3, 3, 5, 5, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    // editor.draw_textarea("Hello\nIt is me", 4, 4);
    // editor.draw_textarea_cut_overflaw("Hello\nIt is not me", 4, 4, 5, 2);
    // editor.draw_bordered_textarea("Hello\nIt is me", 10, 6, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    // editor.draw_bordered_textarea_cut_overflaw("Hello\nIt is me", 10, 6, 7, 3, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    // editor.draw_node(editor.nodes[1], '╗', '╔', '╝', '╚', '─', '─', '║', '║', '╣', '╠', 5);
    // editor.draw_node(editor.nodes[2], '╗', '╔', '╝', '╚', '─', '─', '║', '║', '╣', '╠', 5);
    // editor.draw_nodes('╗', '╔', '╝', '╚', '─', '─', '║', '║', '╣', '╠', 5);
    // editor.draw_connection(1, 1);
    // editor.draw_container("Output".to_string(), 1, 23, 100, 4, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    
    editor.container.run();
    
    // editor.clear_win();
    // let mut container = Container::new(3, 3, 30, 10, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    // let mut rect1 = Rect::new(22, 8, 10, 4, '┐', '┌', '┘', '└', '─', '─', '│', '│');
    // container.add_child(Child::Rect(rect1));
    // container.draw(0, 0, 70, 15);

    // println!("{}", editor.evaluate_node(1).get_print_text());
}