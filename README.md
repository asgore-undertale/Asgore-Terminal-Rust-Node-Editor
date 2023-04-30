A very little simple terminal node editor prototype in Rust

To use:
    Just run the code and enter commans in the terminal

Available commands:
    calc_out {node id} # calc output
    pos {node id} {x} {y} # position node
    con {from node id} {to node id} {input index} # dis/connect nodes
    set_val {node id} {input index} # set input's default value
    add_node {node title}
    del_node {node id}
    save {path and file name with no extension}
    load {path and file name with no extension}
    autosave on
    autosave off