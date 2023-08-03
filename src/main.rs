use blockchain::chain::Block;

mod blockchain;

fn main() {
    //println!("Hello, world!");
   let block = Block::new(1,vec![], "oxxxx".to_string(), "oxxx".to_string());
   println!("response: {:?}", block.serialize_block());
}
