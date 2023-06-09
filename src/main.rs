// use hexgame_rs::hexgame::game::Game;

use sigma_learning_rs::montecarlo_ts::tree::*;
use sigma_learning_rs::montecarlo_ts::evolving_sytem::*;
fn main() {
    // let state: DecreasingNumbers= DecreasingNumbers{
    //     threshold: 0, numbers: vec![1,2,3,4]
    // };
    // let data: DecreasingNumbersData= DecreasingNumbersData{action:1000};

    // let tree : NodeDN = Tree::expand_decreasing_numbers_tree_rec( state, data);
    // println!("{}",Tree::visit_rec(tree, 0));

    let state: DecreasingNumbers = DecreasingNumbers {
        threshold: 0,
        numbers: vec![1, 2, 3, 4],
    };
    let data: DecreasingNumbersData = DecreasingNumbersData { action: 1000 };
    let initial_node: NodeG = NodeG {
        data,
        children: Vec::new(),
    };

    let mut tree = TreeG::<DecreasingNumbers> {
        system: state,
        root: initial_node,
    };
    tree.expand_tree();
    println!("{}", tree.visit());
}
