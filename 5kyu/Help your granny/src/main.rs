extern crate help_your_granny;

#[macro_use] extern crate maplit;

fn main() {
    let friends = [ "A1", "A2", "A3", "A4", "A5" ];
    let fr_towns = hashmap!{ "A1" => "X1", "A2"=> "X2", "A3" => "X3", "A4" => "X4" };
    let dst = hashmap!{ "X1" => 100.0, "X2" => 200.0, "X3" => 250.0, "X4" => 300.0 };
    println!("{}", help_your_granny::tour(&friends, fr_towns, dst));
}